use super::types::*;
use openbrush::traits::Hash;

/// Defines the basic game lifecycle methods.
#[openbrush::trait_definition]
pub trait Lifecycle {
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

    /// closes the game and terminates the contract
    /// can only be done once all the rounds have been played
    /// releases the joining fees (unless penalties are incurred)
    /// emits a relevant event
    #[ink(message, payable)]
    fn end_game(&mut self) -> Result<(), GameError>;
}
