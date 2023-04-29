use ink::primitives::AccountId;
use ink::prelude::vec::Vec;

use crate::traits::types::*;

/// Defines the basic game lifecycle methods.
#[ink::trait_definition]
pub trait Basic {
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
    fn join(&mut self, player: AccountId) -> Result<u8, GameError>;
}
