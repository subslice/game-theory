use super::types::GameError;

#[openbrush::trait_definition]
pub trait GameAdmin {
    #[ink(message)]
    fn add_player_to_game(&mut self) -> Result<(), GameError>;

    #[ink(message)]
    fn play_round_as_player(&mut self) -> Result<(), GameError>;

    #[ink(message)]
    fn reveal_round_as_player(&mut self) -> Result<(), GameError>;
    
    #[ink(message)]
    fn force_complete_round(&mut self) -> Result<(), GameError>;

    #[ink(message)]
    fn force_end_game(&mut self) -> Result<(), GameError>;

    #[ink(message, payable)]
    fn fund_contract(&self) -> Result<(), GameError>;
}
