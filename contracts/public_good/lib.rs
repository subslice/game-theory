#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

pub use self::public_good::{PublicGood, PublicGoodRef};

// noinspection ALL
#[openbrush::contract]
pub mod public_good {
    use game_theory::ensure;
    use game_theory::logics::traits::types::{
        GameConfigs, GameError, GameRound, GameStatus, RoundStatus,
    };
    use game_theory::logics::traits::{admin::*, basic::*, lifecycle::*, utils::*};
    use ink::codegen::EmitEvent;
    use ink::codegen::Env;
    use ink::env::hash::{Blake2x256, HashOutput};
    use ink::prelude::vec::Vec;
    use openbrush::contracts::access_control::{extensions::enumerable::*, only_role};
    use openbrush::{
        modifiers,
        traits::{DefaultEnv, Storage},
    };

    /// Access control roles
    const CREATOR: RoleType = ink::selector_id!("CREATOR");

    /// Events
    #[ink(event)]
    pub struct GameCreated {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        game_hash: Hash,
    }

    #[ink(event)]
    pub struct GameStarted {
        #[ink(topic)]
        game_address: AccountId,
    }

    #[ink(event)]
    pub struct PlayerJoined {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
    }

    #[ink(event)]
    #[derive(Debug)]
    pub struct RoundCommitPlayed {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
        commitment: Hash,
    }

    #[ink(event)]
    pub struct AllPlayersCommitted {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        round_id: u8,
    }

    #[ink(event)]
    pub struct RoundCommitRevealed {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        player: AccountId,
        reveal: Option<(u128, u128)>,
    }

    #[ink(event)]
    pub struct RoundForceClosed {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        round_id: u8,
        // represents the admin which took the action to force close the round
        // is simply the caller of the method in open games
        admin_id: AccountId,
    }

    #[ink(event)]
    pub struct RoundCompleted {
        #[ink(topic)]
        game_address: AccountId,
        #[ink(topic)]
        round_id: u8,
        winners: Vec<(AccountId, Option<u128>)>,
    }

    #[ink(event)]
    pub struct GameEnded {
        #[ink(topic)]
        game_address: AccountId,
    }

    /// A single game storage.
    /// Each contract (along with its storage) represents a single game instance.
    #[ink(storage)]
    #[derive(Storage)]
    pub struct PublicGood {
        /// openbrush access control storage
        #[storage_field]
        access: access_control::Data<Members>,
        created_by: AccountId,
        /// Stores the list of players for this game instance
        players: Vec<AccountId>,
        /// The status of the current game
        status: GameStatus,
        /// The current round of the game
        current_round: Option<GameRound>,
        /// The id of the next round
        next_round_id: u8,
        /// The configurations of the game
        configs: GameConfigs,
    }

    impl PublicGood {
        /// Constructor that initializes the PublicGood struct
        #[ink(constructor)]
        pub fn new(configs: GameConfigs) -> Self {
            // basic sanity checks related to round contributions for this game
            if configs.max_round_contribution.is_none() {
                panic!("The max_round_contribution must be set");
            } else if configs.min_round_contribution.is_none() {
                panic!("The min_round_contribution must be set");
            } else if configs.max_round_contribution.unwrap()
                < configs.min_round_contribution.unwrap()
            {
                panic!(
                    "The max_round_contribution must be greater than the min_round_contribution"
                );
            }

            let mut instance = Self {
                access: Default::default(),
                created_by: <Self as DefaultEnv>::env().caller(),
                players: Vec::new(),
                status: GameStatus::Ready,
                current_round: None,
                next_round_id: 1,
                configs,
            };

            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_admin(caller);
            instance
                .grant_role(CREATOR, caller)
                .expect("Should grant CREATOR role");

            instance
        }

        /// A default constructor that initializes this game with 10 players.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(GameConfigs {
                max_players: 10,
                min_players: 2,
                min_round_contribution: Some(100),
                max_round_contribution: Some(1_000),
                round_reward_multiplier: Some(20),
                post_round_actions: false,
                round_timeout: None,
                max_rounds: Some(3),
                join_fee: None,
                is_rounds_based: false,
            })
        }

        /// Helper methods
        #[modifiers(only_role(CREATOR))]
        pub fn emit_game_created(&mut self) -> Result<(), GameError> {
            let game_address = self.env().account_id();
            let game_hash = <Self as DefaultEnv>::env()
                .code_hash(&game_address)
                .unwrap();

            self.env().emit_event(GameCreated {
                game_address,
                game_hash,
            });

            Ok(())
        }

        #[modifiers(only_role(CREATOR))]
        pub fn emit_game_started(&mut self) -> Result<(), GameError> {
            self.env().emit_event(GameStarted {
                game_address: self.env().account_id(),
            });

            Ok(())
        }

        // TODO: this is would be on the front end
        #[ink(message)]
        pub fn hash_commitment(&self, input: u128, nonce: u128) -> Result<Hash, GameError> {
            let data = [input.to_le_bytes(), nonce.to_le_bytes()].concat();
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut output);
            Ok(output.into())
        }
    }

    /// Add default implementation for access control to the game
    impl AccessControl for PublicGood {}
    impl AccessControlEnumerable for PublicGood {}

    /// An implementation of the basic ink! storage getters / setters
    impl Basic for PublicGood {
        #[ink(message)]
        fn get_configs(&self) -> GameConfigs {
            self.configs.clone()
        }

        #[ink(message)]
        fn get_players(&self) -> Vec<AccountId> {
            self.players.clone()
        }

        #[ink(message)]
        fn get_status(&self) -> GameStatus {
            self.status.clone()
        }

        #[ink(message)]
        fn get_current_round(&self) -> Option<GameRound> {
            self.current_round.clone()
        }

        #[ink(message, payable)]
        fn join(&mut self, player: AccountId) -> Result<u8, GameError> {
            // ensure that joining is only done by caller
            ensure!(
                Self::env().caller() == player,
                GameError::CallerMustMatchNewPlayer
            );
            // ensure that there's more room in the game
            ensure!(
                self.players.len() < self.configs.max_players as usize,
                GameError::MaxPlayersReached
            );
            // ensure applicable fees are paid
            if let Some(fees) = self.configs.join_fee {
                ensure!(
                    Self::env().transferred_value() >= Balance::from(fees),
                    GameError::InsufficientJoiningFees
                );
            }
            // add player to state
            self.players.push(player);
            // emit PlayerJoined event
            Self::env().emit_event(PlayerJoined {
                game_address: Self::env().account_id(),
                player,
            });
            Ok(self.players.len() as u8)
        }
    }

    /// An implementation of the `GameLifecycle` trait for the `PublicGood` contract.
    impl Lifecycle for PublicGood {
        #[ink(message, payable)]
        fn start_game(&mut self) -> Result<(), GameError> {
            // ensure game status is valid for state change
            ensure!(
                self.status == GameStatus::Ready,
                GameError::InvalidGameState
            );
            // ensure enough players
            ensure!(
                self.players.len() >= self.configs.min_players as usize,
                GameError::NotEnoughPlayers
            );

            // setup the current round
            self.current_round = Some(GameRound {
                id: self.next_round_id,
                status: RoundStatus::Ready,
                player_commits: Vec::new(),
                player_reveals: Vec::new(),
                player_contributions: Vec::new(),
                total_contribution: 0,
                total_reward: 0,
            });
            self.next_round_id += 1;
            // update game state
            self.status = GameStatus::OnGoing;
            // emit GameStarted event
            Self::env().emit_event(GameStarted {
                game_address: Self::env().account_id(),
            });
            Ok(())
        }

        #[ink(message, payable)]
        fn play_round(&mut self, commitment: Hash) -> Result<(), GameError> {
            // ensure valid game state
            ensure!(
                self.status == GameStatus::OnGoing,
                GameError::GameNotStarted
            );
            // ensure current round exists
            ensure!(self.current_round.is_some(), GameError::NoCurrentRound);

            let value = Self::env().transferred_value();
            // NOTE: the issue here is since this game is publicgood, some amount has to be
            // contributed to the pot. So, we need to check if the player has contributed
            // that amount. But we also don't want to reveal the contribution :)
            // one way is to have the payable amount always be fixed and be maxed out
            // while the hashed commitment contains the real amount to be contributed.
            ensure!(
                value >= Balance::from(self.configs.max_round_contribution.unwrap_or(0)),
                GameError::InvalidRoundContribution
            );

            let caller = Self::env().caller();
            let current_round = self.current_round.as_mut().unwrap();

            // ensure that the player hasn't already made a commitment
            ensure!(
                current_round
                    .player_commits
                    .iter()
                    .find(|(player, _)| player == &caller)
                    .is_none(),
                GameError::PlayerAlreadyCommitted
            );

            // store the commit
            current_round
                .player_commits
                .push((caller.clone(), commitment));

            // keep track of round contribution(s)
            current_round
                .player_contributions
                .push((caller.clone(), value));

            current_round.total_contribution += value;

            // check if all players have committed
            if current_round.player_commits.len() == self.players.len() {
                Self::env().emit_event(AllPlayersCommitted {
                    game_address: Self::env().account_id(),
                    round_id: current_round.id.clone(),
                });
            }

            Self::env().emit_event(RoundCommitPlayed {
                game_address: Self::env().account_id(),
                player: Self::env().caller(),
                commitment,
            });
            Ok(())
        }

        #[ink(message, payable)]
        fn reveal_round(&mut self, reveal: (u128, u128)) -> Result<(), GameError> {
            let caller = Self::env().caller();
            let data = [reveal.0.to_le_bytes(), reveal.1.to_le_bytes()].concat();
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut output);

            let player_commitment = self
                .current_round
                .as_ref()
                .unwrap()
                .player_commits
                .iter()
                .find(|(player, _)| player == &caller);

            // ensure that the commitment exists
            ensure!(player_commitment.is_some(), GameError::CommitmentNotFound);
            // ensure that the reveal is valid
            ensure!(
                player_commitment.unwrap().1 == output.into(),
                GameError::InvalidReveal
            );

            // return the partial contribution to the player
            // this is done because all players contribute the max amount when making a commitment
            // to avoid information leakage
            Self::env()
                .transfer(
                    caller,
                    self.configs.max_round_contribution.unwrap() - reveal.0,
                )
                .map_err(|_| GameError::PartialContributionRefundFailed)?;

            // store the reveal
            self.current_round
                .as_mut()
                .unwrap()
                .player_reveals
                .push((caller, reveal));
            // emit event
            self.env().emit_event(RoundCommitRevealed {
                game_address: Self::env().account_id(),
                player: Self::env().caller(),
                reveal: Some(reveal),
            });

            Ok(())
        }

        #[ink(message)]
        fn complete_round(&mut self) -> Result<(), GameError> {
            let current_round = self.current_round.as_mut().unwrap();

            // ensure all players have revealed
            ensure!(
                current_round.player_reveals.len() == self.players.len(),
                GameError::NotAllPlayersRevealed
            );
            // ensure round state is still valid
            ensure!(
                current_round.status != RoundStatus::Ended,
                GameError::InvalidGameState
            );

            // mark round as ended
            current_round.status = RoundStatus::Ended;
            // get winners
            let winners = PublicGood::get_winners(&current_round, &self.configs, &self.players)
                .map_err(|err| err)?;

            // issue winner rewards
            winners.iter().for_each(|(player, reward)| match reward {
                Some(reward) => {
                    let _ = Self::env()
                        .transfer(*player, *reward)
                        .map_err(|_| GameError::FailedToIssueWinnerRewards);
                }
                None => (),
            });

            self.env().emit_event(RoundCompleted {
                game_address: Self::env().account_id(),
                round_id: self.current_round.as_ref().unwrap().id,
                winners,
            });

            // TODO: handle checking players who haven't played

            // check if there's a next round or game ended
            if self.configs.max_rounds.unwrap_or(999) < self.next_round_id.into() {
                self.status = GameStatus::Ended;
                Self::env().emit_event(GameEnded {
                    game_address: Self::env().account_id(),
                });
            } else {
                self.current_round = Some(GameRound {
                    id: self.next_round_id,
                    status: RoundStatus::Ready,
                    player_commits: Vec::new(),
                    player_reveals: Vec::new(),
                    player_contributions: Vec::new(),
                    total_contribution: 0,
                    total_reward: 0,
                });
                self.next_round_id += 1;
            }

            // TODO: emit event (new round started)

            Ok(())
        }

        #[ink(message, payable)]
        fn end_game(&mut self) -> Result<(), GameError> {
            // ensure the game is in ended state
            ensure!(
                self.status == GameStatus::Ended,
                GameError::InvalidGameState
            );
            // terminate the contract and send remaining balance to the contract's creator
            Self::env().terminate_contract(self.created_by);
        }
    }

    /// An implementation of the `Utils` trait containing internal helper functions
    impl Utils for PublicGood {
        fn get_winners(
            round: &GameRound,
            configs: &GameConfigs,
            _players: &Vec<AccountId>,
        ) -> Result<Vec<(AccountId, Option<u128>)>, GameError> {
            if round.status != RoundStatus::Ended {
                return Err(GameError::RoundNotEnded);
            }

            // for each player that played, apply the multiplier to their contribution
            let winners: Vec<(AccountId, Option<u128>)> = round
                .player_reveals
                .iter()
                .map(|&(account_id, play)| {
                    (
                        account_id,
                        Some(
                            (play.0 * configs.round_reward_multiplier.unwrap().abs() as u128) / 10,
                        ),
                    )
                })
                .collect();

            Ok(winners)
        }
    }

    /// An implementation of Admin-level functions for the `PublicGood` contract.
    impl Admin for PublicGood {
        #[ink(message, payable)]
        #[modifiers(only_role(CREATOR))]
        fn add_player_to_game(&mut self, player: AccountId) -> Result<u8, GameError> {
            // ensure that there's more room in the game
            ensure!(
                self.players.len() < self.configs.max_players as usize,
                GameError::MaxPlayersReached
            );
            // add player to state
            self.players.push(player);
            // any paid amount should be transferred to that particular player from the contract
            let value = Self::env().transferred_value();
            if value > 0 {
                Self::env()
                    .transfer(player, Self::env().transferred_value())
                    .map_err(|_| GameError::FailedToAddPlayer)?;
            }
            // emit PlayerJoined event
            Self::env().emit_event(PlayerJoined {
                game_address: Self::env().account_id(),
                player,
            });
            Ok(self.players.len() as u8)
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn play_round_as_player(
            &mut self,
            as_player: AccountId,
            commitment: Hash,
        ) -> Result<(), GameError> {
            // TODO: refactor this logic into something re-usable for both admin and player

            // ensure valid game state
            ensure!(
                self.status == GameStatus::OnGoing,
                GameError::GameNotStarted
            );
            // ensure current round exists
            ensure!(self.current_round.is_some(), GameError::NoCurrentRound);

            let value = Self::env().transferred_value();
            // NOTE: the issue of contribution amount privacy is discussed in the `play_round` method implementation.
            // It's the reason we require the max_round_contribution amount here
            ensure!(
                value >= Balance::from(self.configs.max_round_contribution.unwrap_or(0)),
                GameError::InvalidRoundContribution
            );

            let caller = as_player.clone();
            let current_round = self.current_round.as_mut().unwrap();

            // ensure that the player hasn't already made a commitment
            ensure!(
                current_round
                    .player_commits
                    .iter()
                    .find(|(player, _)| player == &caller)
                    .is_none(),
                GameError::PlayerAlreadyCommitted
            );

            // store the commit
            current_round
                .player_commits
                .push((as_player.clone(), commitment));

            // keep track of round contribution(s)
            current_round
                .player_contributions
                .push((as_player.clone(), value));

            current_round.total_contribution += value;

            // check if all players have committed
            if current_round.player_commits.len() == self.players.len() {
                Self::env().emit_event(AllPlayersCommitted {
                    game_address: Self::env().account_id(),
                    round_id: current_round.id.clone(),
                });
            }

            Self::env().emit_event(RoundCommitPlayed {
                game_address: Self::env().account_id(),
                player: caller,
                commitment,
            });
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn reveal_round_as_player(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn force_complete_round(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message)]
        #[modifiers(only_role(CREATOR))]
        fn force_end_game(&mut self) -> Result<(), GameError> {
            todo!("implement")
        }

        #[ink(message, payable)]
        fn fund_contract(&mut self) -> Result<(), GameError> {
            // ensure!(self.env().transferred_value() > 0);

            Ok(())
        }
    }

    /// Unit tests.
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::EmittedEvent;
        use openbrush::traits::Balance;

        type Event = <PublicGood as ::ink::reflect::ContractEventBase>::Type;

        struct SetupTestGame {
            join_game: bool,
            start_game: bool,
            play_commits: bool,
        }

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn get_balance(account: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account).unwrap()
        }

        fn setup_game(configs: SetupTestGame) -> PublicGood {
            let accounts = get_accounts();

            let mut game_public_good = PublicGood::default();

            if configs.join_game {
                set_caller(accounts.alice);
                assert!(game_public_good.join(accounts.alice).is_ok());
                set_caller(accounts.bob);
                assert!(game_public_good.join(accounts.bob).is_ok());
                set_caller(accounts.charlie);
                assert!(game_public_good.join(accounts.charlie).is_ok());
            }

            if configs.start_game {
                assert!(game_public_good.start_game().is_ok());
            }

            if configs.play_commits {
                let mut commitment = <Blake2x256 as HashOutput>::Type::default();
                let data = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
                ink::env::hash_bytes::<Blake2x256>(&data, &mut commitment);

                set_value(game_public_good.configs.max_round_contribution.unwrap());

                set_caller(accounts.alice);
                assert!(game_public_good.play_round(commitment.into()).is_ok());
                set_caller(accounts.bob);
                assert!(game_public_good.play_round(commitment.into()).is_ok());
                set_caller(accounts.charlie);
                assert!(game_public_good.play_round(commitment.into()).is_ok());
            }

            game_public_good
        }

        fn set_caller(account: AccountId) -> () {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        fn set_value(value: Balance) -> () {
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(value);
        }

        /// Default constructor works.
        #[ink::test]
        fn default_works() {
            let game_public_good = PublicGood::default();
            assert_eq!(game_public_good.players, vec![]);
            assert_eq!(game_public_good.get_current_round(), None);
        }

        /// Can construct with "new()" method.
        #[ink::test]
        fn new_works() {
            let game_public_good = PublicGood::new(GameConfigs {
                max_players: 10,
                min_players: 2,
                min_round_contribution: Some(100),
                max_round_contribution: Some(1_000),
                round_reward_multiplier: Some(15),
                post_round_actions: false,
                round_timeout: None,
                max_rounds: None,
                join_fee: None,
                is_rounds_based: false,
            });
            assert_eq!(game_public_good.players, vec![]);
            assert_eq!(game_public_good.get_current_round(), None);
        }

        /// A new player can join the game.
        #[ink::test]
        fn player_can_join() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = PublicGood::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can join when the caller is alice joining as alice (own account)
            assert!(game_public_good.join(accounts.alice).is_ok());
        }

        /// A new player cannot add someone else to the game.
        #[ink::test]
        fn player_must_join_as_self() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = PublicGood::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // can't join when the caller is alice trying to add bob's account
            assert!(game_public_good.join(accounts.bob).is_err());
        }

        /// A player can start the game.
        #[ink::test]
        fn player_can_start_game() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = PublicGood::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(game_public_good.join(accounts.alice).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(game_public_good.join(accounts.bob).is_ok());

            // can start the game when there are enough players
            match game_public_good.start_game() {
                Err(error) => {
                    println!("{:?}", error);
                    assert!(false);
                }
                Ok(_) => assert!(true),
            }
        }

        /// A player cannot start a game that is already started.
        #[ink::test]
        fn player_cannot_start_already_started_game() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = PublicGood::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(game_public_good.join(accounts.alice).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(game_public_good.join(accounts.bob).is_ok());

            // can start the game when there are enough players
            assert!(game_public_good.start_game().is_ok());
            // cannot start again
            assert_eq!(
                game_public_good.start_game().err(),
                Some(GameError::InvalidGameState)
            );
        }

        /// A player cannot start a game that doesn't have enough players.
        #[ink::test]
        fn game_cannot_start_without_enough_players() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let mut game_public_good = PublicGood::default();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(game_public_good.join(accounts.alice).is_ok());

            // cannot start, not enough players
            assert_eq!(
                game_public_good.start_game().err(),
                Some(GameError::NotEnoughPlayers)
            );
        }

        /// A player can play a round.
        #[ink::test]
        fn player_can_play_round() {
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: false,
            });

            let mut commitment = <Blake2x256 as HashOutput>::Type::default();
            let data = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut commitment);

            // can play a round
            set_value(game_public_good.configs.max_round_contribution.unwrap());
            match game_public_good.play_round(commitment.into()) {
                Err(error) => {
                    println!("{:?}", error);
                    assert!(false);
                }
                Ok(_) => assert!(true),
            };

            // round commit is stored
            let commits = game_public_good
                .current_round
                .as_ref()
                .unwrap()
                .player_commits
                .clone();
            assert_eq!(commits.len(), 1);
            assert_eq!(commits.first().unwrap().1, commitment.into());
        }

        /// A player cannot play a round twice.
        #[ink::test]
        fn player_cannot_play_twice() {
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: false,
            });

            let mut commitment = <Blake2x256 as HashOutput>::Type::default();
            let data = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut commitment);

            // can play a round
            set_value(game_public_good.configs.max_round_contribution.unwrap());
            assert!(game_public_good.play_round(commitment.into()).is_ok());

            // cannot play again for the same round
            assert!(game_public_good.play_round(commitment.into()).is_err());
        }

        /// All players can play round.
        #[ink::test]
        fn all_players_can_play_round() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: false,
            });

            let mut commitment = <Blake2x256 as HashOutput>::Type::default();
            let data = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut commitment);

            // can play a round
            set_value(game_public_good.configs.max_round_contribution.unwrap());

            set_caller(accounts.alice);
            assert!(game_public_good.play_round(commitment.into()).is_ok());

            set_caller(accounts.charlie);
            assert!(game_public_good.play_round(commitment.into()).is_ok());

            set_caller(accounts.bob);
            assert!(game_public_good.play_round(commitment.into()).is_ok());
        }

        /// Each player can play the round then reveal.
        #[ink::test]
        fn players_can_play_and_reveal() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: true,
            });

            // do the reveal step for each player
            set_caller(accounts.alice);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            set_caller(accounts.bob);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            set_caller(accounts.charlie);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            // check that all reveals are stored in state
            assert_eq!(
                game_public_good
                    .current_round
                    .as_ref()
                    .unwrap()
                    .player_reveals
                    .len(),
                3
            );
        }

        /// A reveal must match the commitment.
        #[ink::test]
        fn reveal_must_match_commitment() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: true,
            });

            set_caller(accounts.alice);

            // the reveal used below is different from that which is committed to in the "setup_game" function
            match game_public_good.reveal_round((200, 144)) {
                Err(_) => assert!(true),
                Ok(_) => {
                    println!("reveal must be considered invalid");
                    assert!(false);
                }
            };
        }

        /// Players can complete a round.
        #[ink::test]
        fn players_can_complete_round() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: true,
            });

            // do the reveal step for each player
            set_caller(accounts.alice);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            set_caller(accounts.bob);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            set_caller(accounts.charlie);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());

            // attempt to complete the round
            match game_public_good.complete_round() {
                Err(err) => {
                    println!("Error: {:?}", err);
                    assert!(false);
                }
                Ok(_) => {
                    // check that the round ID has been incremented
                    assert_eq!(game_public_good.next_round_id, 3);
                    // check that the relevant round completion event is emitted
                    let events: Vec<EmittedEvent> =
                        ink::env::test::recorded_events().collect::<Vec<_>>();

                    // ensure the relevant event is emitted
                    let mut found: bool = false;
                    for e in events {
                        // decode the event
                        let decoded_event = <Event as scale::Decode>::decode(&mut &e.data[..])
                            .expect("encountered invalid contract event data buffer");

                        // match the event type for the data
                        match decoded_event {
                            Event::RoundCompleted(_data) => {
                                found = true;
                                break;
                            }
                            _ => (),
                        }
                    }

                    // check that the round completion event is emitted
                    assert!(found);
                }
            };
        }

        /// Round cannot be completed if not all players revealed.
        #[ink::test]
        fn all_players_must_reveal_to_complete_round() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: false,
            });

            let mut commitment = <Blake2x256 as HashOutput>::Type::default();
            let data = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
            ink::env::hash_bytes::<Blake2x256>(&data, &mut commitment);

            let mut commitment2 = <Blake2x256 as HashOutput>::Type::default();
            let data2 = [100u128.to_le_bytes(), 144u128.to_le_bytes()].concat();
            ink::env::hash_bytes::<Blake2x256>(&data2, &mut commitment2);

            set_value(game_public_good.configs.max_round_contribution.unwrap());

            set_caller(accounts.alice);
            assert!(game_public_good.play_round(commitment.into()).is_ok());
            set_caller(accounts.bob);
            assert!(game_public_good.play_round(commitment.into()).is_ok());
            set_caller(accounts.charlie);
            assert!(game_public_good.play_round(commitment2.into()).is_ok());

            // do the reveal step for each player
            set_caller(accounts.alice);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());
            set_caller(accounts.charlie);
            assert!(game_public_good.reveal_round((100, 144)).is_ok());

            // attempt to complete the round
            assert_eq!(
                game_public_good.complete_round().err(),
                Some(GameError::NotAllPlayersRevealed)
            );
        }

        #[ink::test]
        fn refunds_are_processed_upon_reveal() {
            let accounts = get_accounts();
            let mut game_public_good = setup_game(SetupTestGame {
                join_game: true,
                start_game: true,
                play_commits: true,
            });

            let contribution_amount = 100;
            let alice_balance = get_balance(accounts.alice);
            let expected_refund =
                game_public_good.configs.max_round_contribution.unwrap() - contribution_amount;

            // do the reveal step for each player
            set_caller(accounts.alice);
            assert!(game_public_good
                .reveal_round((contribution_amount, 144))
                .is_ok());

            // attempt to complete the round
            assert_eq!(get_balance(accounts.alice), alice_balance + expected_refund);
        }

        #[ink::test]
        fn only_contract_creator_can_emit_start_event() {
            let accounts = get_accounts();
            set_caller(accounts.alice);
            let mut game_public_good = PublicGood::default();

            // attempt to emit the start event as bob
            // expect failure
            set_caller(accounts.bob);
            matches!(
                game_public_good.emit_game_started(),
                Err(GameError::AccessControlError(_))
            );

            // attempt to emit the stat event as alice
            // expect success
            set_caller(accounts.alice);
            assert!(game_public_good.emit_game_started().is_ok());
        }
    }

    /// On-chain (E2E) tests.
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        // Default constructor works.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = PublicGoodRef::default();

            // When
            let contract_account_id = client
                .instantiate("game_public_good", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiation failed")
                .account_id;

            // Then
            let get_players = ink_e2e::build_message::<PublicGoodRef>(contract_account_id.clone())
                .call(|test| test.get_players());
            let get_result = client
                .call_dry_run(&ink_e2e::alice(), &get_players, 0, None)
                .await;
            assert_eq!(get_result.return_value(), vec![]);

            Ok(())
        }
    }
}
