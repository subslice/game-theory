use ink::primitives::AccountId;
use ink::prelude::vec::Vec;

use crate::traits::types::*;

pub trait Utils {
    /// helper function to get winners of current round
    fn get_winners(round: &GameRound, configs: &GameConfigs, players: &Vec<AccountId>) -> Result<Vec<(AccountId, Option<u128>)>, GameError>;

    /// helper to check that it's the contract owner, for admin purposes
    /// until implementing OpenBrush
    fn only_owner(&self) -> Result<bool, GameError>;
}
