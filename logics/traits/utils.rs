use openbrush::traits::AccountId;
use ink::prelude::vec::Vec;
use super::types::*;

pub trait Utils {
    /// helper function to get winners of current round
    fn get_winners(round: &GameRound, configs: &GameConfigs, players: &Vec<AccountId>) -> Result<Vec<(AccountId, Option<u128>)>, GameError>;
}
