use super::types::*;
use ink::prelude::vec::Vec;
use openbrush::traits::AccountId;

pub trait Utils {
    /// helper function to get winners of current round
    fn get_winners(
        round: &GameRound,
        configs: &GameConfigs,
        players: &Vec<AccountId>,
    ) -> Result<Vec<(AccountId, Option<u128>)>, GameError>;
}
