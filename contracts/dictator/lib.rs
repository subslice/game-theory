#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract(env = CustomEnvironment)]
mod dictator {
    use core::mem::uninitialized;

    use game_theory::logics::traits::basic::*;
    use game_theory::logics::traits::lifecycle::*;
    use game_theory::logics::traits::types::{CustomEnvironment, RandomReadErr};
    use game_theory::logics::traits::types::{
        GameConfigs, GameError, GameRound, GameStatus, RoundStatus,
    };
    use ink::codegen::EmitEvent;
    use ink::env::hash::{Blake2x256, HashOutput};
    use ink::prelude::vec::Vec;
    use openbrush::contracts::access_control::extensions::enumerable::*;
    use openbrush::contracts::access_control::only_role;
    use openbrush::modifiers;
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
    }

    impl Dictator {
        /// Constructor that initializes the `bool` value to the given `init_value`.
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
                configs,
            };
            let caller = <Self as DefaultEnv>::env().caller();

            instance._init_with_admin(caller);
            instance
                .grant_role(CREATOR, caller)
                .expect("Should grant CREATOR role");

            instance
        }

        /// Default constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            unimplemented!();
        }

        /// Seed a random value by passing some known argument `subject` to the runtime's
        /// random source. Then, update the current `value` stored in this contract with the
        /// new random value.
        #[ink(message)]
        pub fn update(&mut self, subject: [u8; 32]) -> Result<[u8; 32], RandomReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random(subject)?;
            // Emit the `RandomUpdated` event when the random seed
            // is successfully fetched.
            Ok(new_random)
        }
    }

    // impl Lifecycle for Dictator {
    // }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mut dictator = Dictator::default();
            assert_eq!(dictator.get(), [0u8; 32]);
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
