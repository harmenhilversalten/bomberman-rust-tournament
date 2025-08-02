use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use events::{bus::EventBus, events::BotDecision};
use tokio::task::JoinHandle;

use bot::{
    AiType, Bot as KernelBot, BotConfig, BotState, DecisionMaker, HeuristicAI, PlanningAI,
    ReactiveAI,
};
use state::grid::GridDelta;

use events::events::bot_events::BotId;

/// Errors related to bot management.
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    /// Invalid configuration provided.
    #[error("invalid bot configuration: {0}")]
    InvalidConfig(String),
    /// Bot with the given id was not found.
    #[error("bot not found")]
    NotFound,
}

/// Handle to a running bot task.
pub struct BotHandle {
    /// Identifier of the bot.
    pub id: BotId,
    join: JoinHandle<BotState>,
}

impl BotHandle {
    /// Abort the running bot task.
    pub fn abort(&self) {
        self.join.abort();
    }
}

/// Manager responsible for spawning and tracking bots.
pub struct BotManager {
    runtime: tokio::runtime::Runtime,
    next_id: AtomicUsize,
}

impl BotManager {
    /// Create a new [`BotManager`].
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("runtime");
        Self {
            runtime,
            next_id: AtomicUsize::new(0),
        }
    }

    fn build_ai(&self, ai: AiType) -> Box<dyn DecisionMaker<GridDelta, BotDecision>> {
        match ai {
            AiType::Heuristic => Box::new(HeuristicAI),
            AiType::Reactive => Box::new(ReactiveAI),
            AiType::Planning => Box::new(PlanningAI),
        }
    }

    /// Spawn a bot using the provided configuration.
    pub fn spawn_bot(
        &self,
        mut config: BotConfig,
        bus: Arc<EventBus>,
    ) -> Result<BotHandle, BotError> {
        config
            .validate()
            .map_err(|e| BotError::InvalidConfig(e.to_string()))?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        config.id = id;
        let ai = self.build_ai(config.ai_type);
        let bot = KernelBot::new(config, bus, ai);
        let join = self.run_bot_decision_loop(bot);
        Ok(BotHandle { id, join })
    }

    /// Run the decision loop for a bot asynchronously.
    pub fn run_bot_decision_loop(&self, bot: KernelBot) -> JoinHandle<BotState> {
        self.runtime.spawn(async move { bot.run() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawns_bot_and_returns_handle() {
        let manager = BotManager::new();
        let bus = Arc::new(EventBus::new());
        let cfg = BotConfig::new("b", AiType::Heuristic);
        let handle = manager.spawn_bot(cfg, bus).expect("spawn");
        assert_eq!(handle.id, 0);
        handle.abort();
    }
}
