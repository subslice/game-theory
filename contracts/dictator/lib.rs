#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract(env = CustomEnvironment)]
mod dictator {
    use game_theory::logics::traits::basic::*;
    use game_theory::logics::traits::{ basic::*, lifecycle::*, utils::*, admin::* };
    use game_theory::logics::traits::types::{CustomEnvironment, RandomReadErr};
    use openbrush::{modifiers, traits::{DefaultEnv, Storage}};
    use game_theory::logics::traits::types::{
        GameConfigs, GameError, GameRound, GameStatus, RoundStatus,
    };
    use ink::codegen::{Env};
    use game_theory::ensure;
    use ink::env::hash::{Blake2x256, HashOutput};
    use ink::prelude::vec::Vec;
    
    use ink_env::debug_println;
    use openbrush::contracts::access_control::{extensions::enumerable::*, only_role};
    
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
    #[derive(Debug)]
    pub struct RoundCommitPlayed {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
        commitment: Hash,
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

    #[ink(event)]
    pub struct DictatorChosen {
        #[ink(topic)]
        dictator: AccountId,
        endowment: Balance,
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
        seed: Option<[u8; 32]>,
        /// current round dictator
        dictator: Option<AccountId>,
        /// endowment each player will receive in the current round
        current_prize: Option<u128>,
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
                seed: None,
                dictator: None,
                current_prize: None,
            };
            let caller = <Self as DefaultEnv>::env().caller();

            // if let Some(max_rounds) = configs.max_rounds {
            //     if max_rounds > 1 {
            //         panic!("Dictator is not rounds based")
            //     }
            // }

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
                min_round_contribution: Some(100_000),
                max_round_contribution: Some(1_000_000),
                round_reward_multiplier: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: Some(1),
                join_fee: None,
                is_rounds_based: false,
            })
        }

        #[ink(message)]
        pub fn set_random_seed(&mut self, seed: [u8; 32]) -> Result<[u8; 32], GameError> {
            self.seed = Some(seed);

            Ok(seed)
        }

        #[ink(message)]
        pub fn update(&mut self, subject: [u8; 32]) -> Result<[u8; 32], RandomReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random(subject)?;
            // self.value = new_random;
            // Emit the `RandomUpdated` event when the random seed
            // is successfully fetched.
            Ok(new_random)
        }

        // /// Return the last stored random value
        // #[ink(message)]
        // pub fn get(&mut self) -> [u8; 32] {
        //     self.value
        // }
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

            ensure!(self.configs.min_round_contribution.is_some(), GameError::InvalidRoundContribution);
            ensure!(self.env().balance() > self.configs.min_round_contribution.unwrap(), GameError::BalanceNotEnough);

            ensure!(self.seed.is_some(), GameError::SeedNotSet);
            let new_random = self.env().extension().fetch_random(self.seed.unwrap()).unwrap();
            let rand_int = u32::from_ne_bytes(new_random[0..4].try_into().unwrap());
            let idx = rand_int as usize % self.players.len();
            let dictator = self.players.get(idx).unwrap();
            self.dictator = Some(*dictator);

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), DictatorChosen {
                dictator: *dictator,
                endowment: self.env().balance(),
            });

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

            current_round.player_commits.push((caller, commitment));

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
            let caller = Self::env().caller();
            let data = [reveal.0.to_le_bytes(), reveal.1.to_le_bytes()].concat();
            let mut output = <Blake2x256 as HashOutput>::Type::default(); // 256 bit buffer
            ink_env::hash_bytes::<Blake2x256>(&data, &mut output);

            let mut current_round = self.current_round.clone().unwrap();
            
            // we check if the dictator prize to each player
            // sums up to the total endowment
            if self.env().caller() == self.dictator.unwrap() {
                let prize = reveal.0;
                if prize * self.players.len() as u128 > self.env().balance() {
                    return Err(GameError::InvalidReveal);
                }
                self.current_prize = Some(prize);
            }

            let player_commitment = current_round
                .player_commits
                .iter()
                .find(|(c, _)| c == &caller);
            if let Some(commit) = player_commitment {
                if commit.1 != output.into() {
                    return Err(GameError::InvalidReveal);
                }
            } else {
                return Err(GameError::CommitmentNotFound);
            }

            current_round.player_reveals.push((caller, reveal));

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), PlayerRevealed {
                game_address: Self::env().account_id(),
                player: caller,
                reveal,
            });

            self.current_round = Some(current_round);

            Ok(())

        }

        #[ink(message, payable)]
        fn complete_round(&mut self) -> Result<(), GameError> {
            let mut current_round = self.current_round.clone().unwrap();

            if current_round.player_reveals.len() != self.players.len() {
                return Err(GameError::NotAllPlayersRevealed);
            };

            if current_round.status != RoundStatus::OnGoing {
                return Err(GameError::InvalidRoundState);
            };

            current_round.status = RoundStatus::Ended;

            ensure!(self.current_prize.is_some(), GameError::EndowmentNotSet);
            let current_prize = self.current_prize.unwrap();

            let mut receivers: Vec<(AccountId, u128)> = Vec::new();

            for (caller, reveal) in current_round.player_reveals.iter() {
                if reveal.0 == 1 {
                    let recv = (*caller, current_prize);
                    self.env().transfer(recv.0, recv.1).map_err(|_| GameError::FailedToIssueWinnerRewards)?;
                    receivers.push(recv);
                }
            }

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), RoundEnded {
                game_address: Self::env().account_id(),
                winners: receivers,
                round_id: self.next_round_id,
                total_contribution: self.current_prize.unwrap(),
            });

            self.current_round = Some(current_round);

            Ok(())
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

    impl GameAdmin for Dictator {
        #[ink(message, payable)]
        #[modifiers(only_role(CREATOR))]
        fn add_player_to_game(&mut self, player: AccountId) -> Result<u8, GameError> {
            // ensure that there's more room in the game
            ensure!(self.players.len() < self.configs.max_players as usize, GameError::MaxPlayersReached);
            // add player to state
            self.players.push(player);
            // any paid amount should be transferred to that particular player from the contract
            let value = Self::env().transferred_value();
            if value > 0 {
                Self::env().transfer(player, Self::env().transferred_value())
                    .map_err(|_| GameError::FailedToAddPlayer)?;
            }
            // emit PlayerJoined event
            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), PlayerJoined {
                game_address: Self::env().account_id(),
                player,
            });
            Ok(self.players.len() as u8)
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn play_round_as_player(&mut self, as_player: AccountId, commitment: Hash) -> Result<(), GameError> {
            // TODO: refactor this logic into something re-usable for both admin and player

            // ensure valid game state
            ensure!(self.status == GameStatus::OnGoing, GameError::GameNotStarted);
            // ensure current round exists
            ensure!(self.current_round.is_some(), GameError::NoCurrentRound);

            let value = Self::env().transferred_value();
            // NOTE: the issue of contribution amount privacy is discussed in the `play_round` method implementation.
            // It's the reason we require the max_round_contribution amount here
            ensure!(value >= Balance::from(self.configs.max_round_contribution.unwrap_or(0)), GameError::InvalidRoundContribution);

            let caller = as_player.clone();
            let mut current_round = self.current_round.clone().unwrap();

            // ensure that the player hasn't already made a commitment
            ensure!(
                current_round.player_commits.iter().find(|(player, _)| player == &caller).is_none(),
                GameError::PlayerAlreadyCommitted
            );

            // store the commit
            current_round.player_commits.push((
                as_player.clone(),
                commitment,
            ));

            // keep track of round contribution(s)
            current_round.player_contributions.push((
                as_player.clone(),
                value,
            ));

            current_round.total_contribution += value;

            // check if all players have committed
            if current_round.player_commits.len() == self.players.len() {
                ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), AllPlayersCommitted {
                    game_address: Self::env().account_id(),
                    commits: current_round.player_commits.clone(),
                });
            }

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), RoundCommitPlayed {
                game_address: Self::env().account_id(),
                player: caller,
                commitment,
            });

            self.current_round = Some(current_round);

            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn reveal_round_as_player(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn force_complete_round(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn force_end_game(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn fund_contract(&mut self) -> Result<(), GameError> {
            let value = self.env().transferred_value();
            ensure!(value > self.configs.min_round_contribution.unwrap(), GameError::EndowmentNotEnough);

            ink::codegen::EmitEvent::<Dictator>::emit_event(self.env(), GameEndowmentDeposited {
                creator: self.env().caller(),
                game_address: self.env().account_id(),
                endowment: self.env().transferred_value()
            });

            Ok(())
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
