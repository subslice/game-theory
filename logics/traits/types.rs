use openbrush::traits::{AccountId, Hash};
use scale::{Decode, Encode};
use ink::prelude::vec::Vec;
use ink::storage::traits::StorageLayout;

#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GameError {
    FailedToEmitEvent,
    FailedToGetWinners,
    /// Caller must match the palyer being added
    CallerMustMatchNewPlayer,
    /// No more space for players to join
    MaxPlayersReached,
    /// Fees paid to join the game are not sufficient
    InsufficientJoiningFees,
    /// The round has not expired yet
    RoundNotExpired,
    /// No commitment made by player for the current round
    CommitmentNotFound,
    /// The commitment doesn't match the revealed value
    InvalidReveal,
    /// Round cannot be closed
    FailedToCloseRound,
    /// The game hasn't reached enough players
    NotEnoughPlayers,
    /// Game status isn't set to Started
    GameNotStarted,
    /// The current round has not been set, i.e. game hasn't started
    NoCurrentRound,
    /// Invalid round state
    InvalidRoundState,
    /// The current round hasn't ended yet
    RoundNotEnded,
    /// Invalid state to start the game with
    InvalidGameState,
    /// Invalid value payed to play a round
    InvalidRoundContribution,
    /// Partial contribution refund transfer failed
    PartialContributionRefundFailed,
    /// Not all the players revealed
    NotAllPlayersRevealed,
    /// Failed to issue winner rewards
    FailedToIssueWinnerRewards,
    /// Player is already in the game
    PlayerAlreadyJoined,
    /// Player already played n this round
    PlayerAlreadyCommitted,
    /// Player choice for the round is not valid
    InvalidChoice
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum GameStatus {
    Ready,
    OnGoing,
    Ended,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum RoundStatus {
    Ready,
    OnGoing,
    Ended,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub struct GameRound {
    pub id: u8,
    pub status: RoundStatus,
    pub player_commits: Vec<(AccountId, Hash)>,
    pub player_reveals: Vec<(AccountId, (u128, u128))>,
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
    /// The multiplier for the round reward. Always divisible by 10 to allow for decimal values.
    pub round_reward_multiplier: Option<i8>,
    pub post_round_actions: bool,
    /// The number of blocks before a round is considered stale.
    pub round_timeout: Option<u32>,
    pub max_rounds: Option<u32>,
    pub join_fee: Option<u128>,
    pub is_rounds_based: bool,
}
