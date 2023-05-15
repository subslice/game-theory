use super::types::GameError;
use ink::primitives::Hash;
use openbrush::traits::AccountId;

/// Defines a set of method signatures for admin control of the game.
///
/// One example is being able to add players into the game directly without restriction.
#[openbrush::trait_definition]
pub trait GameAdmin {
    /// Adds a player into the game on their behalf.
    ///
    /// Any amount paid into this method call is sent to the player being added to the game.
    /// - This is optional and serves as an initial deposit to the players.
    ///
    /// This method must have access restriction such that only the contract deployer/admin
    /// can call it
    #[ink(message, payable)]
    fn add_player_to_game(&mut self, player: AccountId) -> Result<u8, GameError>;

    /// Make a commitment to a round on behalf of a player.
    #[ink(message)]
    fn play_round_as_player(
        &mut self,
        as_player: AccountId,
        commitment: Hash,
    ) -> Result<(), GameError>;

    /// Reveal a commitment on behalf of a player.
    #[ink(message)]
    fn reveal_round_as_player(&mut self) -> Result<(), GameError>;

    /// Mark a round as completed and distribute rewards even if not all players have completed.
    ///
    /// This could optionally require an expiry (i.e. certain # of blocks passing).
    #[ink(message)]
    fn force_complete_round(&mut self) -> Result<(), GameError>;

    /// Close the current round using `force_complete_round` and proceed and end the entire
    /// game without the completion of any remaining rounds.
    #[ink(message)]
    fn force_end_game(&mut self) -> Result<(), GameError>;

    /// A general helper method to put funds into the game's contract as the deployer/admin.
    #[ink(message, payable)]
    fn fund_contract(&mut self) -> Result<(), GameError>;
}
