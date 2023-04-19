#![cfg_attr(not(feature = "std"), no_std)]

pub use self::game_rock_paper_scissors::{GameRockPaperScissors, GameRockPaperScissorsRef};

#[ink::contract]
pub mod game_rock_paper_scissors {
    use ink::prelude::vec::Vec;
    use traits::{GameConfigs, GameError, GameLifecycle, GameRound, GameStatus};

    /// A single game storage.
    /// Each contract (along with its storage) represents a single game instance.
    #[ink(storage)]
    pub struct GameRockPaperScissors {
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
        /// The configurations of the game
        configs: GameConfigs,
    }

    impl GameRockPaperScissors {
        /// Constructor that initializes the GameRockPaperScissors struct
        #[ink(constructor)]
        pub fn new(configs: GameConfigs) -> Self {
            Self {
                players: Vec::new(),
                status: GameStatus::Initialized,
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
                min_round_contribution: None,
                max_round_contribution: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
                is_rounds_based: false,
            })
        }
    }

    impl GameLifecycle for GameRockPaperScissors {
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

            if self.players.len() >= self.configs.max_players as usize {
                return Err(GameError::MaxPlayersReached);
            }

            if let Some(fees) = self.configs.join_fee {
                if self.env().transferred_value() < fees {
                    return Err(GameError::InsufficientJoiningFees);
                }
            }

            self.players.push(player);
            Ok(self.players.len() as u8)
        }

        #[ink(message, payable)]
        fn start_game(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn play_round(&mut self, commitment: Hash) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn reveal_round(&mut self, reveal: (u128, u128)) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn complete_round(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn force_complete_round(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn end_game(&mut self) -> Result<(), GameError> {
            todo!("implement")
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
            let game_rock_paper_scissors = GameRockPaperScissors::default();
            assert_eq!(game_rock_paper_scissors.players, vec![]);
            assert_eq!(game_rock_paper_scissors.get_current_round(), None)
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn new_works() {
            let game_rock_paper_scissors = GameRockPaperScissors::new(GameConfigs {
                max_players: 2,
                min_players: 2,
                min_round_contribution: None,
                max_round_contribution: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
                is_rounds_based: false,
            });

            assert_eq!(game_rock_paper_scissors.players, vec![]);
            assert_eq!(game_rock_paper_scissors.get_current_round(), None);
        }

        /// A new player can join the game.
        #[ink::test]
        fn player_can_join() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_rock_paper_scissors = GameRockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(game_rock_paper_scissors.join(accounts.alice).is_ok());
        }

        /// A new player cannot add someone else to the game.
        #[ink::test]
        fn player_must_join_as_self() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_rock_paper_scissors = GameRockPaperScissors::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can't join when the caller is alice trying to add bob's account
            assert!(game_rock_paper_scissors.join(accounts.bob).is_err());
        }
    }
}
