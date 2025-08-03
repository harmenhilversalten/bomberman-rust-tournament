mod game_session;
mod registry;
mod scheduler;
mod scoring;

use game_session::GameSession;
use registry::BotRegistry;
use scheduler::GameScheduler;
use scoring::{BotScore, ScoreTracker};

use crate::{config::TournamentConfig, config::UnifiedBotConfig as BotConfig, SystemHandle};
use events::events::bot_events::BotId;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GameResult {
    pub winner: BotId,
    pub participants: Vec<BotId>,
    pub survival_times: HashMap<BotId, Duration>,
    pub destruction_points: HashMap<BotId, u32>,
    pub powerups_collected: HashMap<BotId, u32>,
}

impl GameResult {
    pub fn new(participants: Vec<BotId>, winner: BotId) -> Self {
        Self {
            winner,
            participants,
            survival_times: HashMap::new(),
            destruction_points: HashMap::new(),
            powerups_collected: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResultAggregator {
    pub results: Vec<GameResult>,
}

impl ResultAggregator {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_results(&mut self, res: Vec<GameResult>) {
        self.results.extend(res);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TournamentState {
    Idle,
    Registration,
    Running,
    Completed,
}

#[derive(Debug, thiserror::Error)]
pub enum TournamentError {
    #[error("invalid state for operation")]
    InvalidState,
    #[error("registration closed")]
    RegistrationClosed,
    #[error("game failed: {0}")]
    GameFailed(String),
}

pub struct TournamentResults {
    pub rankings: Vec<(BotId, BotScore, u32)>,
}

pub struct TournamentManager {
    _config: TournamentConfig,
    state: TournamentState,
    bot_registry: BotRegistry,
    game_scheduler: GameScheduler,
    score_tracker: ScoreTracker,
    result_aggregator: ResultAggregator,
    system_handle: SystemHandle,
}

impl TournamentManager {
    pub fn new(config: TournamentConfig, system_handle: SystemHandle) -> Self {
        let scheduler = GameScheduler::new(config.format.clone());
        let tracker = ScoreTracker::new(config.scoring_system.clone());
        Self {
            _config: config,
            state: TournamentState::Idle,
            bot_registry: BotRegistry::default(),
            game_scheduler: scheduler,
            score_tracker: tracker,
            result_aggregator: ResultAggregator::new(),
            system_handle,
        }
    }

    pub async fn start_registration(&mut self) -> Result<(), TournamentError> {
        if self.state != TournamentState::Idle {
            return Err(TournamentError::InvalidState);
        }
        self.state = TournamentState::Registration;
        Ok(())
    }

    pub async fn register_bot(&mut self, bot_config: BotConfig) -> Result<BotId, TournamentError> {
        if self.state != TournamentState::Registration {
            return Err(TournamentError::RegistrationClosed);
        }
        self.bot_registry.register_bot(bot_config)
    }

    pub async fn start_tournament(&mut self) -> Result<(), TournamentError> {
        if self.state != TournamentState::Registration {
            return Err(TournamentError::InvalidState);
        }
        self.state = TournamentState::Running;
        Ok(())
    }

    pub fn has_next_round(&self) -> bool {
        self.game_scheduler.has_next_round()
    }

    pub async fn run_next_round(&mut self) -> Result<Vec<GameResult>, TournamentError> {
        if self.state != TournamentState::Running {
            return Err(TournamentError::InvalidState);
        }
        let bots = self.bot_registry.get_bot_ids();
        let matches = self.game_scheduler.schedule_next_round(&bots);
        let mut results = Vec::new();
        for m in matches {
            let mut session = GameSession::new(m.id, m.participants.clone());
            session.start(&self.system_handle).await?;
            let res = session.wait_for_completion().await?;
            results.push(res);
        }
        self.score_tracker.update_scores(&results);
        self.result_aggregator.add_results(results.clone());
        Ok(results)
    }

    pub async fn finalize_tournament(&mut self) -> Result<TournamentResults, TournamentError> {
        if self.state != TournamentState::Running {
            return Err(TournamentError::InvalidState);
        }
        self.state = TournamentState::Completed;
        let rankings = self.score_tracker.get_rankings();
        Ok(TournamentResults { rankings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::*, engine::Engine};
    use events::bus::EventBus;
    use std::sync::{Arc, RwLock};

    fn dummy_handle() -> SystemHandle {
        let cfg = EngineConfig::default();
        let grid = Arc::new(RwLock::new(state::GameGrid::new(5, 5)));
        let (engine, _) = Engine::with_components(cfg, grid, Arc::new(EventBus::new()));
        SystemHandle::new(Arc::new(EventBus::new()), engine, 0, None)
    }

    #[test]
    fn full_flow() {
        let config = TournamentConfig {
            name: "test".into(),
            format: TournamentFormat::RoundRobin { total_rounds: 1 },
            max_concurrent_games: 1,
            game_timeout_seconds: 60,
            scoring_system: ScoringSystem::WinLoss {
                win_points: 1,
                loss_points: 0,
            },
            registration_timeout_seconds: 1,
            allow_remote_bots: false,
            persist_results: false,
        };
        let handle = dummy_handle();
        let mut tm = TournamentManager::new(config, handle);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            tm.start_registration().await.unwrap();
            let bot_cfg = BotConfig {
                name: "b1".into(),
                ai_type: "Heuristic".into(),
                rl_mode: false,
                rl_model_path: None,
                decision_timeout_ms: 10,
            };
            tm.register_bot(bot_cfg.clone()).await.unwrap();
            tm.register_bot(bot_cfg).await.unwrap();
            tm.start_tournament().await.unwrap();
            while tm.has_next_round() {
                let res = tm.run_next_round().await.unwrap();
                assert!(!res.is_empty());
            }
            let finals = tm.finalize_tournament().await.unwrap();
            assert_eq!(finals.rankings.len(), 2);
        });
    }
}
