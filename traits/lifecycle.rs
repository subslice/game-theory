#![cfg_attr(not(feature = "std"), no_std)]

use ink::primitives::{AccountId, Hash};
use ink::prelude::vec::Vec;
use ink::storage::traits::StorageLayout;

use crate::traits::types::*;

/// Defines the basic game lifecycle methods.
#[openbrush::trait_definition]
pub trait Lifecycle {
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

    /// callable by any player who joined
    /// only works once, fails on subsequent calls (since the state has changed)
    /// emits a relevant event (all events should include some game ID for the UIs that are listening)
    #[ink(message, payable)]
    fn start_game(&mut self) -> Result<(), GameError>;

    /// Makes a commitment to the current round by the player who called the method
    /// The payed amount is the round contribution, to be validated based on configs
    /// Must be recorded in the GameRound storage
    /// emits a relevant event (should include the total # of commitments in the round, helps UI know if everyone played)
    #[ink(message, payable)]
    fn play_round(&mut self, commitment: Hash) -> Result<(), GameError>;

    /// receives data which if hashed must match the commitment for the round made earlier
    /// throws an error if the round has no commitment for the caller
    /// prepares the next round if max rounds not reached
    /// emits a relevant event
    #[ink(message, payable)]
    fn reveal_round(&mut self, reveal: (u128, u128)) -> Result<(), GameError>;

    /// claims rewards of the round (if applicable and all players have revealed)
    /// prepares the next round
    /// emits a relevant event
    #[ink(message, payable)]
    fn complete_round(&mut self) -> Result<(), GameError>;

    /// succeeds only if the caller has already made a commitment
    /// succeeds only if the round expired (passed the block timeout in config // should default to 10 or 20 blocks if None)
    /// a penalty is incurred by the players who did not play (joining fee is not returned)
    /// emits a relevant event
    #[ink(message, payable)]
    fn force_complete_round(&mut self) -> Result<(), GameError>;

    /// closes the game and terminates the contract
    /// can only be done once all the rounds have been played
    /// releases the joining fees (unless penalties are incurred)
    /// emits a relevant event
    #[ink(message, payable)]
    fn end_game(&mut self) -> Result<(), GameError>;
}
