#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
mod router {
    use dictator::DictatorRef;
    use ink::storage::Mapping;
    use public_good::PublicGoodRef;
    use rock_paper_scissors::RockPaperScissorsRef;

    /// Game types.
    #[derive(scale::Encode, scale::Decode, PartialEq, Eq, Clone, Copy, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum Game {
        RockPaperScissors,
        PublicGood,
        Dictator,
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
            let game_hash = self.get_game_hash(which)?;

            self.games_count += 1;

            match which {
                Game::RockPaperScissors => {
                    RockPaperScissorsRef::default()
                        .code_hash(game_hash)
                        .endowment(self.env().transferred_value())
                        .gas_limit(0)
                        .salt_bytes(self.games_count.to_le_bytes())
                        .instantiate();
                }
                Game::PublicGood => {
                    PublicGoodRef::default()
                        .code_hash(game_hash)
                        .endowment(self.env().transferred_value())
                        .gas_limit(0)
                        .salt_bytes(self.games_count.to_le_bytes())
                        .instantiate();
                }
                Game::Dictator => {
                    DictatorRef::default()
                        .code_hash(game_hash)
                        .endowment(self.env().transferred_value())
                        .gas_limit(0)
                        .salt_bytes(self.games_count.to_le_bytes())
                        .instantiate();
                }
            }

            Ok(())
        }
    }
}
