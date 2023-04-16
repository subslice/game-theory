#![cfg_attr(not(feature = "std"), no_std)]

pub use self::game_public_good::{GamePublicGood, GamePublicGoodRef};

#[ink::contract]
pub mod game_public_good {
    use traits::{ GameLifecycle, GameRound, GameStatus, GameConfigs, Error };
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
        /// The configurations of the game
        configs: GameConfigs,
    }

    impl GamePublicGood {
        /// Constructor that initializes the `bool` value to the given `init_value`.
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

        /// A default constructor that initializes this game with 10 players.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(GameConfigs {
                max_players: 10,
                min_players: 2,
                min_round_contribution: None,
                max_round_contribution: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
            })
        }
    }

    /// An implementation of the `GameLifecycle` trait for the `GamePublicGood` contract.
    impl GameLifecycle for GamePublicGood {
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
            }
            
            if self.players.len() >= self.configs.max_players as usize {
                return Err(Error::MaxPlayersReached)
            }

            match self.configs.join_fee {
                Some(fees) => {
                    if self.env().transferred_value() < fees {
                        return Err(Error::InsufficientJoiningFees)
                    }
                }
                None => (),
            }

            self.players.push(player);
            Ok(self.players.len() as u8)
        }
    }

    /// Unit tests.
    #[cfg(test)]
    mod tests {
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
            let game_public_good = GamePublicGood::new(GameConfigs {
                max_players: 10,
                min_players: 2,
                min_round_contribution: None,
                max_round_contribution: None,
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
            });
            assert_eq!(game_public_good.players, vec![]);
            assert_eq!(game_public_good.get_current_round(), None);
        }

        /// A new player can join the game.
        #[ink::test]
        fn player_can_join() {
            let accounts = 
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = GamePublicGood::default();
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(game_public_good.join(accounts.alice).is_ok());
        }

        /// A new player cannot add someone else to the game.
        #[ink::test]
        fn player_must_join_as_self() {
            let accounts = 
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = GamePublicGood::default();
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can't join when the caller is alice trying to add bob's account
            assert!(game_public_good.join(accounts.bob).is_err());
        }
    }

    /// On-chain (E2E) tests.
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        // Default constructor works.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = GamePublicGoodRef::default();

            // When
            let contract_account_id = client
                .instantiate("game_public_good", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiation failed")
                .account_id;

            // Then
            let get_players = build_message::<GamePublicGoodRef>(contract_account_id.clone())
                .call(|test| test.get_players());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get_players, 0, None).await;
            assert_eq!(get_result.return_value(), vec![]);

            Ok(())
        }
    }
}
