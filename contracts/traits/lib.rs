#![cfg_attr(not(feature = "std"), no_std)]

use ink::primitives::AccountId;
use ink::prelude::vec::Vec;
use ink::storage::traits::StorageLayout;
use scale::{Encode, Decode};

// TODO: add some events

/// Game errors.
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Caller must match the palyer being added
    CallerMustMatchNewPlayer,
    /// No more space for players to join
    MaxPlayersReached,
    /// Fees paid to join the game are not sufficient
    InsufficientJoiningFees,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum GameStatus {
    Initialized,
    Ready,
    Started,
    Ended,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum RoundStatus {
    Ready,
    Started,
    PendingRewardsClaim,
    Ended,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct GameRound {
    pub round_number: u32,
    pub status: RoundStatus,
    pub player_commits: Vec<(AccountId, u128)>,
    pub player_reveals: Vec<(AccountId, u128)>,
    pub player_contributions: Vec<(AccountId, u128)>,
    pub total_contribution: u128,
    pub total_reward: u128,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct GameConfigs {
    pub max_players: u8,
    pub min_players: u8,
    pub min_round_contribution: Option<u128>,
    pub max_round_contribution: Option<u128>,
    pub post_round_actions: bool,
    /// The number of blocks before a round is considered stale.
    pub round_timeout: Option<u32>,
    pub max_rounds: Option<u32>,
    pub join_fee: Option<u128>,
}

/// Defines the basic game lifecycle methods.
#[ink::trait_definition]
pub trait GameLifecycle {
    /// Gets the AccountId of each each player within this instance of the game.
    #[ink(message)]
    fn get_configs(&self) -> GameConfigs;
    
    /// Gets the AccountId of each each player within this instance of the game.
    #[ink(message)]
    fn get_players(&self) -> Vec<AccountId>;

    /// Get the status of the current game.
    #[ink(message)]
    fn get_status(&self) -> GameStatus;

    /// Get the current game round.
    #[ink(message)]
    fn get_current_round(&self) -> Option<GameRound>;

    /// Adds a player into the game by their AccountId.
    /// Ensures that caller of the function has the same AccountId being added (i.e. player can add themselves).
    /// 
    /// Returns the number of players.
    #[ink(message, payable)]
    fn join(&mut self, player: AccountId) -> Result<u8, Error>;
}
