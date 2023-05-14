#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract(env = CustomEnvironment)]
mod dictator {
    use game_theory::logics::traits::basic::*;
    use game_theory::logics::traits::lifecycle::*;
    use game_theory::logics::traits::types::{CustomEnvironment, RandomReadErr};
    use game_theory::logics::traits::types::{
        GameConfigs, GameError, GameRound, GameStatus, RoundStatus,
    };
    use ink::codegen::{Env};
    use ink::env::hash::{Blake2x256, HashOutput};
    use ink::prelude::vec::Vec;
    
    use openbrush::contracts::access_control::extensions::enumerable::*;
    
    
    use openbrush::traits::{DefaultEnv, Storage};

    /// Access control roles
    const CREATOR: RoleType = ink::selector_id!("CREATOR");

    #[ink(event)]
    pub struct GameCreated {
        #[ink(topic)]
        game_id: AccountId,
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        configs: GameConfigs,
    }

    #[ink(event)]
    pub struct GameStarted {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        players: Vec<AccountId>,
    }

    #[ink(event)]
    pub struct GameEnded {
        #[ink(topic)]
        game_address: AccountId,
        rounds_played: u8,
    }

    #[ink(event)]
    pub struct PlayerJoined {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
    }

    #[ink(event)]
    pub struct PlayerCommitted {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
        commitment: Hash,
    }

    #[ink(event)]
    pub struct AllPlayersCommitted {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        commits: Vec<(AccountId, Hash)>,
    }

    #[ink(event)]
    pub struct PlayerRevealed {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
        reveal: (u128, u128),
    }

    #[ink(event)]
    pub struct RoundEnded {
        #[ink(topic)]
        game_address: AccountId,
        winners: Vec<(AccountId, u128)>,
        round_id: u8,
        total_contribution: u128,
    }

    #[ink(event)]
    pub struct GameEndowmentDeposited {
        #[ink(topic)]
        creator: AccountId,
        #[ink(topic)]
        game_address: AccountId,
        endowment: u128,
    }

    /// Storage of the Dictator Game
    #[ink(storage)]
    #[derive(Storage)]
    pub struct Dictator {
        #[storage_field]
        access: access_control::Data<enumerable::Members>,
        /// Stores the list of players for this game instance
        players: Vec<AccountId>,
        /// The status of the current game
        status: GameStatus,
        /// A list of all the rounds that have been played
        rounds: Vec<GameRound>,
        /// The current round of the game
        current_round: Option<GameRound>,
        /// The id of the next round
        next_round_id: u8,
        /// Account that created the instance of this game
        creator: AccountId,
        /// The configurations of the game
        configs: GameConfigs,
        /// current random seed
        seed: [u8; 32],
    }

    impl Dictator {
        #[ink(constructor)]
        pub fn new(configs: GameConfigs) -> Self {
            let mut instance = Self {
                access: Default::default(),
                creator: <Self as DefaultEnv>::env().caller(),
                players: Vec::new(),
                status: GameStatus::Ready,
                rounds: Vec::new(),
                current_round: None,
                next_round_id: 1,
                configs: configs.clone(),
                seed: [0u8; 32]
            };
            let caller = <Self as DefaultEnv>::env().caller();

            if let Some(max_rounds) = configs.max_rounds {
                if max_rounds > 1 {
                    panic!("Dictator is not rounds based")
                }
            }

            instance._init_with_admin(caller);
            instance
                .grant_role(CREATOR, caller)
                .expect("Should grant CREATOR role");

            instance
        }

        /// Default constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(GameConfigs {
                max_players: 2,
                min_players: 2,
                min_round_contribution: Some(100),
                max_round_contribution: Some(1_000),
                round_reward_multiplier: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: Some(1),
                join_fee: None,
                is_rounds_based: false,
            })
        }

        #[ink(message)]
        pub fn make_deposit(&mut self) -> Result<(), GameError> {
            if self.env().transferred_value() < 100 {
                return Err(GameError::InsufficientJoiningFees)
            }

            self.env().emit_event(GameEndowmentDeposited {
                creator: self.env().caller(),
                game_address: self.env().account_id(),
                endowment: self.env().transferred_value()
            });

            Ok(())
        }

        #[ink(message)]
        pub fn set_random_seed(&mut self, seed: [u8; 32]) -> Result<[u8; 32], GameError> {
            self.seed = seed;

            Ok(seed)
        }
    }

    impl Basic for Dictator {
        #[ink(message)]
        fn get_configs(&self) -> GameConfigs {
            self.configs.clone()
        }

        #[ink(message)]
        fn get_players(&self) -> Vec<AccountId> {
            self.players.clone()
        }

        #[ink(message)]
        fn get_status(&self) -> GameStatus {
            self.status
        }

        #[ink(message)]
        fn get_current_round(&self) -> Option<GameRound> {
            self.current_round.clone()
        }

        #[ink(message, payable)]
        fn join(&mut self, player: AccountId) -> Result<u8, GameError> {
            if Self::env().caller() != player {
                return Err(GameError::CallerMustMatchNewPlayer);
            }

            if self.players.len() >= self.configs.max_players.into() {
                return Err(GameError::MaxPlayersReached);
            }

            if self.players.contains(&player) {
                return Err(GameError::PlayerAlreadyJoined);
            };

            if let Some(fees) = self.configs.join_fee {
                if Self::env().transferred_value() < fees {
                    return Err(GameError::InsufficientJoiningFees);
                }
            }

            self.players.push(player);

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), PlayerJoined {
                game_address: Self::env().account_id(),
                player: Self::env().caller(),
            });

            Ok(self.players.len() as u8)
        }
    }

    impl Lifecycle for Dictator {
        #[ink(message, payable)]
        fn start_game(&mut self) -> Result<(), GameError> {
            if self.players.len() < self.configs.min_players.into() {
                return Err(GameError::NotEnoughPlayers);
            }

            if self.status != GameStatus::Ready {
                return Err(GameError::InvalidGameState);
            }

            self.current_round = Some(GameRound {
                id: self.next_round_id,
                status: RoundStatus::Ready,
                player_commits: Vec::new(),
                player_reveals: Vec::new(),
                player_contributions: Vec::new(),
                total_contribution: 0,
                total_reward: 0,
            });

            self.status = GameStatus::OnGoing;
            self.next_round_id += 1;

            let new_random = self.env().extension().fetch_random(self.seed).unwrap();
            // let r = u8::from_le_bytes(new_random);

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), GameStarted {
                game_address: Self::env().account_id(),
                players: self.players.clone(),
            });

            Ok(())
        }

        #[ink(message, payable)]
        fn play_round(&mut self, commitment: Hash) -> Result<(), GameError> {
            if self.status != GameStatus::OnGoing {
                return Err(GameError::InvalidGameState);
            }

            if self.current_round.is_none() {
                return Err(GameError::NoCurrentRound);
            }

            // Here we need to clone Self and then re-bind the current_round
            // in the end of the method. Necessary so we can borrow self.env()
            // immutably
            let mut current_round = self.current_round.clone().unwrap();
            // let current_round = self.current_round.as_mut().unwrap();
            if current_round.status == RoundStatus::Ready {
                current_round.status = RoundStatus::OnGoing
            }

            let caller = Self::env().caller();

            if let Some(_p) = current_round
                .player_commits
                .iter()
                .find(|(c, _)| c == &caller) {
                    return Err(GameError::PlayerAlreadyCommitted);
                }

            if let Some(min_round_contribution) = self.configs.min_round_contribution {
                if Self::env().transferred_value() < min_round_contribution {
                    return Err(GameError::InvalidRoundContribution);
                }
            }

            let value = Self::env().transferred_value();

            // current_round.player_contributions.push((caller, value));
            // current_round.player_commits.push((caller, commitment));
            // current_round.total_contribution += value;

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), PlayerCommitted {
                game_address: Self::env().account_id(),
                player: caller,
                commitment,
            });

            // check if all players have committed
            if current_round.player_commits.len() == self.players.len() {
                ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), AllPlayersCommitted {
                    game_address: Self::env().account_id(),
                    commits: current_round.player_commits.clone(),
                })
            }

            // binding new current_round
            self.current_round = Some(current_round);

            Ok(())
        }

        #[ink(message, payable)]
        fn reveal_round(&mut self, reveal: (u128, u128)) -> Result<(), GameError> {
            unimplemented!()
        }

        #[ink(message, payable)]
        fn complete_round(&mut self) -> Result<(), GameError> {
            let current_round = self.current_round.as_mut().unwrap();

            if current_round.player_reveals.len() != self.players.len() {
                return Err(GameError::NotAllPlayersRevealed);
            };

            if current_round.status != RoundStatus::OnGoing {
                return Err(GameError::InvalidRoundState);
            };

            let player1 = current_round.player_reveals[0];
            let player2 = current_round.player_reveals[1];

            let rewards = current_round.total_contribution;

            let mut winners = Vec::new();

            current_round.status = RoundStatus::Ended;

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), RoundEnded {
                game_address: Self::env().account_id(),
                winners,
                round_id: self.next_round_id,
                total_contribution: rewards,
            });

            Ok(())
        }

        #[ink(message, payable)]
        fn force_complete_round(&mut self) -> Result<(), GameError> {
            // admin level functions necessary
            todo!("implement")
        }

        #[ink(message, payable)]
        fn end_game(&mut self) -> Result<(), GameError> {
            let current_round = self.current_round.as_mut().unwrap();

            if self.status != GameStatus::OnGoing {
                return Err(GameError::InvalidGameState);
            }

            if current_round.status != RoundStatus::Ended {
                return Err(GameError::InvalidRoundState);
            }

            self.status = GameStatus::Ended;

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), GameEnded {
                game_address: Self::env().account_id(),
                rounds_played: self.next_round_id,
            });

            Self::env().terminate_contract(self.creator);
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mut dictator = Dictator::default();
            // assert_eq!(dictator.get(), [0u8; 32]);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = DictatorRef::default();

            // When
            let contract_account_id = client
                .instantiate("dictator", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<DictatorRef>(contract_account_id.clone())
                .call(|dictator| dictator.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = DictatorRef::new(false);
            let contract_account_id = client
                .instantiate("dictator", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<DictatorRef>(contract_account_id.clone())
                .call(|dictator| dictator.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<DictatorRef>(contract_account_id.clone())
                .call(|dictator| dictator.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<DictatorRef>(contract_account_id.clone())
                .call(|dictator| dictator.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
