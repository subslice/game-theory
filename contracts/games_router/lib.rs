#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod games_router {
    use ink::storage::Mapping;
    use game_rock_paper_scissors::GameRockPaperScissorsRef;
    use game_public_good::GamePublicGoodRef;

    #[derive(scale::Encode, scale::Decode, PartialEq, Eq, Clone, Copy, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum WhichGame {
        RockPaperScissors,
        PublicGood,
    }

    #[ink(storage)]
    pub struct GamesRouter {
        owner: AccountId,
        /// The `games` field a mapping of game contract hashes to game contract addresses.
        games: Mapping<Hash, AccountId>,
        /// Keep track of the number of games.
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

    impl GamesRouter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                games: Mapping::new(),
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

        /// A methods that adds a game and instantiates its contract.
        #[ink(message)]
        pub fn new_game(&mut self, game_hash: Hash) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can add games.");
            
        }
    }
}
