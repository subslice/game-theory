use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Hash,
        Storage
    }
};
use crate::traits::types::{
    GameConfigs,
    GameRound,
    GameStatus,
};
use ink::prelude::vec::Vec;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub created_by: AccountId,
    /// Stores the list of players for this game instance
    pub players: Vec<AccountId>,
    /// The status of the current game
    pub status: GameStatus,
    /// The current round of the game
    pub current_round: Option<GameRound>,
    /// The id of the next round
    pub next_round_id: u8,
    /// The configurations of the game
    pub configs: GameConfigs,
}

// impl Default for Data {
//     fn default() -> Self {
//         Self {
//             created_by: AccountId::from(0),
//             players: Vec::new(),
//             status: GameStatus::default(),
//             current_round: None,
//             next_round_id: 1,
//             configs: GameConfigs::default(),
//         }
//     }
// }
