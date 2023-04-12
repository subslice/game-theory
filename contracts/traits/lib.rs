#![cfg_attr(not(feature = "std"), no_std)]

use ink::primitives::AccountId;

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GameStatus {
    Initialized,
    Ready,
    Started,
    Ended,
}

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RoundStatus {
    Ready,
    Started,
    PendingRewardsClaim,
    Ended,
}

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct GameRound {
    pub round_number: u32,
    pub status: RoundStatus,
    pub players: Vec<AccountId>,
    pub player_contributions: Vec<(AccountId, u128)>,
    pub total_contribution: u128,
    pub total_reward: u128,
}

/// Defines the basic game lifecycle methods.
#[ink::trait_definition]
pub trait GameLifecycle {
    /// ...
    #[ink(message, payable)]
    fn initialize(&mut self);

    /// Gets the AccountId of each each player within this instance of the game.
    #[ink(message)]
    fn get_players(&self) -> Vec<AccountId>;

    /// Get the status of the current game.
    #[ink(message)]
    fn get_status(&self) -> GameStatus;

    /// Get the current game round.
    #[ink(message)]
    fn get_current_round(&self) -> GameRound;

    /// Adds a player into the game by their AccountId.
    /// Ensures that caller of the function has the same AccountId being added (i.e. player can add themselves).
    #[ink(message, payable)]
    fn join(&mut self, player: AccountId) -> bool;
}
