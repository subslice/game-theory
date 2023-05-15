#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod router {
    // use public_good::PublicGoodRef;
    // use rock_paper_scissors::RockPaperScissorsRef;
    use ink::storage::Mapping;

    /// Game types.
    #[derive(scale::Encode, scale::Decode, PartialEq, Eq, Clone, Copy, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum Game {
        RockPaperScissors,
        PublicGood,
    }

    /// Router errors.
    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RouterError {
        FailedToInstantiateGame,
        HashNotFoundForGame,
        MustBeOwnerToSetGameHash,
    }

    #[ink(storage)]
    pub struct Router {
        owner: AccountId,
        game_hashes: Mapping<Game, Hash>,
        games_count: u32,
    }

    /**
     *
     * IMPLEMENTATION NOTES:
     *
     * 1. Code in this contract must not contain enums representing game types.
     *    Instead, it should have a storage of the following:
     *      - Vector of allowed Contract Hashes (can only add to it by this contract's owner)
     *      - HashMap of <Contract Hash, Vec<Contract Address>>.
     *
     * 2. Code in this contract must not contain any game-specific logic.
     *
     * 3. Code in this contract must enable instantiation of any game contract using the contract's hash.
     *
     */

    impl Router {
        /// Helper method to ensure that the caller is the contract owner.
        fn ensure_owner(&self) -> Result<(), RouterError> {
            if self.env().caller() != self.owner {
                return Err(RouterError::MustBeOwnerToSetGameHash);
            }

            Ok(())
        }

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                game_hashes: Mapping::new(),
                games_count: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_game_hash(&self, which: Game) -> Result<Hash, RouterError> {
            self.game_hashes
                .get(&which)
                .ok_or(RouterError::HashNotFoundForGame)
        }

        #[ink(message, payable)]
        pub fn set_game_hash(&mut self, which: Game, hash: Hash) -> Result<(), RouterError> {
            self.ensure_owner()?;
            self.game_hashes.insert(which, &hash);
            Ok(())
        }

        /// A methods that adds a game and instantiates its contract.
        #[ink(message, payable)]
        pub fn new_game(&mut self, which: Game) -> Result<(), RouterError> {
            let _game_hash = self.get_game_hash(which)?;

            self.games_count += 1;

            // match which {
            //     Game::RockPaperScissors => {
            //         let game = RockPaperScissorsRef::default()
            //             .code_hash(game_hash)
            //             .endowment(self.env().transferred_value())
            //             .gas_limit(0)
            //             .salt_bytes(self.games_count.to_le_bytes())
            //             .instantiate();

            //         Ok(())
            //     }
            //     Game::PublicGood => {
            //         let game = PublicGoodRef::default()
            //             .code_hash(game_hash)
            //             .endowment(self.env().transferred_value())
            //             .gas_limit(0)
            //             .salt_bytes(self.games_count.to_le_bytes())
            //             .instantiate();

            //         Ok(())
            //     }
            // }
            Ok(())
        }
    }
}
