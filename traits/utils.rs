#![cfg_attr(not(feature = "std"), no_std)]

use ink::primitives::AccountId;
use ink::prelude::vec::Vec;

use crate::traits::types::*;

pub trait Utils {
    /// helper function to get winners of current round
    fn get_winners(round: &GameRound, configs: &GameConfigs, players: &Vec<AccountId>) -> Result<Vec<(AccountId, Option<u128>)>, GameError>;
}
