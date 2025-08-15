#![forbid(unsafe_code)]
#![allow(clippy::all)]

pub mod bot;
pub mod config;
pub mod display;
pub mod engine;
pub mod simulation;
pub mod systems;
pub mod tournament;

use std::sync::{Arc, RwLock};

use events::bus::EventBus;
use state::GameGrid;

pub use ::bot::BotConfig as BotRuntimeConfig;
pub use bot::{BotError, BotHandle, BotManager};
pub use config::{
    AIConfig, BombConfig, ConfigError, EngineConfig, EventBusConfig, GameRules, LoggingConfig,
    RLConfig, TournamentConfig, UnifiedBotConfig, UnifiedConfig,
};
pub use engine::game_engine::EngineError;
pub use engine::{Engine, TaskScheduler};
pub use simulation::{DeterminismChecker, Replay, ReplayRecorder};
pub use systems::System;
pub use tournament::TournamentManager;

/// Errors that may occur during system initialization.
#[derive(Debug, thiserror::Error)]
pub enum InitializationError {
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("bot initialization error: {0}")]
    Bot(String),
    #[error("engine initialization failed")]
    Engine,
}

/// Handle returned after successful system initialization.
pub struct SystemHandle {
    event_bus: Arc<EventBus>,
    engine: Engine,
    bot_count: usize,
    tournament: Option<TournamentConfig>,
}

impl SystemHandle {
    fn new(
        event_bus: Arc<EventBus>,
        engine: Engine,
        bot_count: usize,
        tournament: Option<TournamentConfig>,
    ) -> Self {
        Self {
            event_bus,
            engine,
            bot_count,
            tournament,
        }
    }

    /// Access the initialized engine.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Consume the handle returning the engine.
    pub fn into_engine(self) -> Engine {
        self.engine
    }

    /// Check if a tournament is configured.
    pub fn has_tournament(&self) -> bool {
        self.tournament.is_some()
    }

    /// Access tournament configuration if present.
    pub fn tournament_config(&self) -> Option<&TournamentConfig> {
        self.tournament.as_ref()
    }

    /// Access the event bus.
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Number of bots initialized.
    pub fn bot_count(&self) -> usize {
        self.bot_count
    }
}

/// Initializes the entire Bomberman system from a unified configuration.
pub struct SystemInitializer {
    config: UnifiedConfig,
    event_bus: Option<Arc<EventBus>>,
    game_grid: Option<Arc<RwLock<GameGrid>>>,
    engine: Option<Engine>,
    bot_count: usize,
}

impl SystemInitializer {
    /// Create a new initializer from configuration.
    pub fn new(config: UnifiedConfig) -> Self {
        Self {
            config,
            event_bus: None,
            game_grid: None,
            engine: None,
            bot_count: 0,
        }
    }

    /// Run the initialization pipeline.
    pub async fn initialize(&mut self) -> Result<SystemHandle, InitializationError> {
        self.initialize_event_bus().await?;
        self.initialize_game_state().await?;
        self.initialize_engine().await?;
        self.initialize_ai_components().await?;
        self.initialize_bots().await?;
        let tournament = self.config.tournament.clone();
        Ok(SystemHandle::new(
            Arc::clone(self.event_bus.as_ref().unwrap()),
            self.engine.take().unwrap(),
            self.bot_count,
            tournament,
        ))
    }

    async fn initialize_event_bus(&mut self) -> Result<(), InitializationError> {
        let bus = EventBus::new();
        self.event_bus = Some(Arc::new(bus));
        Ok(())
    }

    async fn initialize_game_state(&mut self) -> Result<(), InitializationError> {
        let grid = GameGrid::new(self.config.engine.width, self.config.engine.height);
        self.game_grid = Some(Arc::new(RwLock::new(grid)));
        Ok(())
    }

    async fn initialize_engine(&mut self) -> Result<(), InitializationError> {
        let events = Arc::clone(self.event_bus.as_ref().ok_or(InitializationError::Engine)?);
        let grid = Arc::clone(self.game_grid.as_ref().ok_or(InitializationError::Engine)?);
        let (mut engine, _rx) =
            engine::Engine::with_components(self.config.engine.clone(), grid, events);
        
        // Add the bomb system for bomb explosions
        engine.add_system(Box::new(systems::BombSystem::new()));
        
        self.engine = Some(engine);
        Ok(())
    }

    async fn initialize_ai_components(&mut self) -> Result<(), InitializationError> {
        // Placeholder for AI component initialization
        Ok(())
    }

    async fn initialize_bots(&mut self) -> Result<(), InitializationError> {
        use ::bot::AiType;
        let engine = self.engine.as_mut().ok_or(InitializationError::Engine)?;
        println!("🤖 Spawning {} bots...", self.config.bots.len());
        for cfg in &self.config.bots {
            let mut bot_cfg = ::bot::BotConfig::new(
                &cfg.name,
                match cfg.ai_type.to_lowercase().as_str() {
                    "reactive" => AiType::Reactive,
                    "planning" => AiType::Planning,
                    _ => AiType::Heuristic,
                },
            );
            bot_cfg.rl_mode = cfg.rl_mode;
            bot_cfg.rl_model_path = cfg.rl_model_path.clone();
            bot_cfg.decision_timeout = std::time::Duration::from_millis(cfg.decision_timeout_ms);
            if let Err(e) = engine.spawn_bot(bot_cfg) {
                println!("❌ Failed to spawn bot {}: {}", cfg.name, e);
                return Err(InitializationError::Bot(e.to_string()));
            }
            self.bot_count += 1;
        }
        println!("✅ Successfully spawned {} bots", self.bot_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_system_with_single_bot() {
        let cfg = UnifiedConfig {
            engine: EngineConfig::default(),
            event_bus: EventBusConfig {
                buffer_size: 1,
                max_subscribers: 1,
            },
            bots: vec![],
            tournament: None,
            ai: AIConfig::default(),
            rl: None,
            bombs: BombConfig::default(),
            logging: LoggingConfig {
                level: "info".into(),
            },
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.block_on(async {
            let mut init = SystemInitializer::new(cfg);
            init.initialize().await.unwrap()
        });
        assert!(!handle.has_tournament());
    }
}
