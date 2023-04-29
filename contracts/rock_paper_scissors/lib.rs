#![cfg_attr(not(feature = "std"), no_std)]

pub use self::rock_paper_scissors::{RockPaperScissors, RockPaperScissorsRef};

#[openbrush::contract]
pub mod rock_paper_scissors {
    use game_theory::logics::traits::types::{GameRound, GameStatus, GameConfigs, GameError, RoundStatus};
    use game_theory::logics::traits::lifecycle::*;
    use game_theory::logics::traits::basic::*;
    use game_theory::logics::traits::utils::*;
    use game_theory::ensure;
    use ink::prelude::vec::Vec;
    use ink::env::hash::{Blake2x256, HashOutput};
    use openbrush::traits::{DefaultEnv, Storage};
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;

    enum Choice {
        Rock,     // 0
        Paper,    // 1
        Scissors, // 2
    }

    #[ink(event)]
    pub struct GameCreated {
        #[ink(topic)]
        game_id: Hash,
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
        commits: Vec<(AccountId, Hash)>
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

    /// A single game storage.
    /// Each contract (along with its storage) represents a single game instance.
    #[ink(storage)]
    #[derive(Storage)]
    pub struct RockPaperScissors {
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
    }

    impl RockPaperScissors {
        /// Constructor that initializes the RockPaperScissors struct
        #[ink(constructor)]
        pub fn new(configs: GameConfigs) -> Self {
            // let game_address = Self::env().account_id();
            // let game_id = Self::env().code_hash(&game_address).unwrap();
            // let creator = Self::env().caller();

            // Self::env().emit_event(GameCreated {
            //     game_id,
            //     creator,
            //     configs: configs.clone(),
            // });

            Self {
                creator: <Self as DefaultEnv>::env().caller(),
                players: Vec::new(),
                status: GameStatus::Ready,
                rounds: Vec::new(),
                current_round: None,
                next_round_id: 1,
                configs,
            }
        }

        /// A default constructor that initializes this game with 2 players
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(GameConfigs {
                max_players: 2,
                min_players: 2,
                min_round_contribution: Some(1),
                max_round_contribution: Some(10000),
                round_reward_multiplier: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
                is_rounds_based: false,
            })
        }

        // testing purposes only
        // this operation should be done by the UI/frontend
        #[ink(message)]
        pub fn hash_commitment(&self, input: u128, nonce: u128) -> Result<Hash, GameError> {
            let data = [input.to_le_bytes(), nonce.to_le_bytes()].concat();
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut output);
            Ok(output.into())
        }

    }

    impl Basic for RockPaperScissors {
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
                return Err(GameError::PlayerAlreadyJoined)
            };

            if let Some(fees) = self.configs.join_fee {
                if Self::env().transferred_value() < fees {
                    return Err(GameError::InsufficientJoiningFees);
                }
            }

            self.players.push(player);

            Self::env().emit_event(PlayerJoined {
                game_address: Self::env().account_id(),
                player: Self::env().caller(),
            });

            Ok(self.players.len() as u8)
        }
    }

    impl Lifecycle for RockPaperScissors {
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

            Self::env().emit_event(GameStarted {
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

            let current_round = self.current_round.as_mut().unwrap();
            if current_round.status == RoundStatus::Ready {
                current_round.status = RoundStatus::OnGoing
            }

            let caller = Self::env().caller();

            if let Some(p) = current_round
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

            current_round.player_contributions.push((caller, value));
            current_round.player_commits.push((caller, commitment));
            current_round.total_contribution += value;

            Self::env().emit_event(PlayerCommitted {
                game_address: Self::env().account_id(),
                player: caller,
                commitment,
            });

            // check if all players have committed
            if current_round.player_commits.len() == self.players.len() {
                Self::env().emit_event(AllPlayersCommitted {
                    game_address: Self::env().account_id(),
                    commits: current_round.player_commits.clone(),
                })
            }

            Ok(())
        }

        #[ink(message, payable)]
        fn reveal_round(&mut self, reveal: (u128, u128)) -> Result<(), GameError> {
            if reveal.0 < 0 || reveal.0 > 2 {
                return Err(GameError::InvalidChoice)
            }

            let caller = Self::env().caller();
            let data = [reveal.0.to_le_bytes(), reveal.1.to_le_bytes()].concat();
            let mut output = <Blake2x256 as HashOutput>::Type::default(); // 256 bit buffer
            ink_env::hash_bytes::<Blake2x256>(&data, &mut output);
            let current_round = self.current_round.as_mut().unwrap();

            let player_commitment = current_round
                .player_commits
                .iter()
                .find(|(c, _)| c == &caller);

            if let Some(c) = player_commitment {
                if c.1 != output.into() {
                    return Err(GameError::InvalidReveal);
                }
            } else {
                return Err(GameError::CommitmentNotFound);
            }

            current_round.player_reveals.push((caller, reveal));

            Self::env().emit_event(PlayerRevealed {
                game_address: Self::env().account_id(),
                player: caller,
                reveal,
            });

            Ok(())
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
            let score = (player1.1 .0 - player2.1 .0) % 3;

            match score {
                1 => {
                    Self::env().transfer(player1.0, rewards);
                    winners.push((player1.0, rewards))
                }
                2 => {
                    Self::env().transfer(player2.0, rewards);
                    winners.push((player2.0, rewards))
                }
                0 => {
                    self.next_round_id += 1;
                    // send event empty vector of winners
                    return Ok(());
                }

                _ => return Err(GameError::FailedToCloseRound),
            }

            current_round.status = RoundStatus::Ended;

            Self::env().emit_event(RoundEnded {
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

            Self::env().emit_event(GameEnded {
                game_address: Self::env().account_id(),
                rounds_played: self.next_round_id,
            });

            Self::env().terminate_contract(self.creator);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let rock_paper_scissors = RockPaperScissors::default();
            assert_eq!(rock_paper_scissors.players, vec![]);
            assert_eq!(rock_paper_scissors.get_current_round(), None)
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn new_works() {
            let rock_paper_scissors = RockPaperScissors::new(GameConfigs {
                max_players: 2,
                min_players: 2,
                min_round_contribution: None,
                max_round_contribution: None,
                round_reward_multiplier: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
                is_rounds_based: false,
            });

            assert_eq!(rock_paper_scissors.players, vec![]);
            assert_eq!(rock_paper_scissors.get_current_round(), None);
        }

        /// A new player can join the game.
        #[ink::test]
        fn player_can_join() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut rock_paper_scissors = RockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(rock_paper_scissors.join(accounts.alice).is_ok());
        }

        /// A new player cannot add someone else to the game.
        #[ink::test]
        fn player_must_join_as_self() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut rock_paper_scissors = RockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can't join when the caller is alice trying to add bob's account
            assert!(rock_paper_scissors.join(accounts.bob).is_err());
        }

        #[ink::test]
        fn player_cannot_join_twice() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut rock_paper_scissors = RockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(rock_paper_scissors.join(accounts.alice).is_ok());
            assert_eq!(rock_paper_scissors.join(accounts.alice), Err(GameError::PlayerAlreadyJoined));
        }

        #[ink::test]
        fn start_game_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut rock_paper_scissors = RockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(rock_paper_scissors.join(accounts.alice).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(rock_paper_scissors.join(accounts.bob).is_ok());

            let result = rock_paper_scissors.start_game();
            assert_eq!(result, Ok(()));
        }

        #[ink::test]
        fn play_round_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut rock_paper_scissors = RockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(rock_paper_scissors.join(accounts.alice).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(rock_paper_scissors.join(accounts.bob).is_ok());

            let result = rock_paper_scissors.start_game();
            assert_eq!(result, Ok(()));

            let alice_data = [0_u128.to_le_bytes(), 69_u128.to_le_bytes()].concat();
            let mut commitment = <Blake2x256 as HashOutput>::Type::default();
            ink_env::hash_bytes::<Blake2x256>(&alice_data, &mut commitment);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                rock_paper_scissors
                    .configs
                    .min_round_contribution
                    .unwrap(),
            );

            let result = rock_paper_scissors.play_round(commitment.into());

            assert_eq!(result, Ok(()));
        }

        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2d_tests {
            use super::*;
            use ink_e2e::build_message;

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let constructor = RockPaperScissorsRef::default();

                // When
                let contract_account_id = client
                    .instantiate(
                        "rock_paper_scissors",
                        &ink_e2e::alice(),
                        constructor,
                        0,
                        None,
                    )
                    .await
                    .expect("instantiation failed")
                    .account_id;

                // Then
                let get_players =
                    ink_e2e::build_message::<RockPaperScissorsRef>(contract_account_id.clone())
                        .call(|test| test.get_players());
                let get_result = client
                    .call_dry_run(&ink_e2e::alice(), &get_players, 0, None)
                    .await;
                assert_eq!(get_result.return_value(), vec![]);

                Ok(())
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2Result<()> {}
        }
    }
}
