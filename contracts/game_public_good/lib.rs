#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod game_public_good {
    use traits::{ GameLifecycle, GameRound, GameStatus, Error };
    use ink::prelude::vec::Vec;

    /// A single game storage.
    /// Each contract (along with its storage) represents a single game instance.
    #[ink(storage)]
    pub struct GamePublicGood {
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
        /// Max players allowed in the game
        max_players: u8,
    }

    impl GamePublicGood {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(max_players: u8) -> Self {
            Self {
                players: Vec::new(),
                status: GameStatus::Initialized,
                rounds: Vec::new(),
                current_round: None,
                next_round_id: 1,
                max_players,
            }
        }

        /// A default constructor that initializes this game with 10 players.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(10)
        }
    }

    /// An implementation of the `GameLifecycle` trait for the `GamePublicGood` contract.
    impl GameLifecycle for GamePublicGood {
        #[ink(message, payable)]
        fn initialize(&mut self) {
            unimplemented!()
        }

        #[ink(message)]
        fn get_players(&self) -> Vec<AccountId> {
            self.players.clone()
        }

        #[ink(message)]
        fn get_status(&self) -> GameStatus {
            self.status.clone()
        }

        #[ink(message)]
        fn get_current_round(&self) -> Option<GameRound> {
            self.current_round.clone()
        }

        #[ink(message, payable)]
        fn join(&mut self, player: AccountId) -> Result<u8, Error> {
            if Self::env().caller() != player {
                return Err(Error::CallerMustMatchNewPlayer)
            } else if self.players.len() >= self.max_players as usize {
                return Err(Error::MaxPlayersReached)
            }

            self.players.push(player);
            Ok(self.players.len() as u8)
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Default constructor works.
        #[ink::test]
        fn default_works() {
            let game_public_good = GamePublicGood::default();
            assert_eq!(game_public_good.players, vec![]);
            assert_eq!(game_public_good.get_current_round(), None);
        }

        /// Can construct with "new()" method.
        #[ink::test]
        fn new_works() {
            let game_public_good = GamePublicGood::new(0);
            assert_eq!(game_public_good.players, vec![]);
            assert_eq!(game_public_good.get_current_round(), None);
        }

        /// A new player can join the game.
        #[ink::test]
        fn player_can_join() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = GamePublicGood::new(10);
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(game_public_good.join(accounts.alice).is_ok());
        }

        /// A new player cannot add someone else to the game.
        #[ink::test]
        fn player_must_join_as_self() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = GamePublicGood::new(10);
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can't join when the caller is alice trying to add bob's account
            assert!(game_public_good.join(accounts.bob).is_err());
        }
    }
}
