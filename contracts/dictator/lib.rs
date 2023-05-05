#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract(env = crate::CustomEnvironment)]
mod dictator {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dictator {
        /// Stores a single `bool` value on the storage.
        value: [u8; 32],
    }

    impl Dictator {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: [u8; 32]) -> Self {
            Self { value: init_value }
        }

        /// Seed a random value by passing some known argument `subject` to the runtime's
        /// random source. Then, update the current `value` stored in this contract with the
        /// new random value.
        #[ink(message)]
        pub fn get_random_value(&mut self, subject: [u8; 32]) -> Result<(), crate::RandomReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random(subject)?;
            self.value = new_random;
            // Emit the `RandomUpdated` event when the random seed
            // is successfully fetched.
            Ok(())
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
            let dictator = Dictator::default();
            assert_eq!(dictator.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut dictator = Dictator::new(false);
            assert_eq!(dictator.get(), false);
            dictator.flip();
            assert_eq!(dictator.get(), true);
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

#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = RandomReadErr;

    #[ink(extension = 12)]
    fn fetch_random(subject: [u8; 32]) -> [u8; 32];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
    FailGetRandomSource,
}

impl ink::env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

use ink_env::Environment;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchRandom;
}