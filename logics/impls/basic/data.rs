use ink::{
    prelude::vec::Vec,
    primitives::Hash,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    created_by: AccountId,
    /// Stores the list of players for this game instance
    players: Vec<AccountId>,
    /// The status of the current game
    status: GameStatus,
    /// The current round of the game
    current_round: Option<GameRound>,
    /// The id of the next round
    next_round_id: u8,
    /// The configurations of the game
    configs: GameConfigs
}