use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use events::bus::EventBus;
use tokio::task::JoinHandle;

use bot::{Bot as KernelBot, BotConfig, BotState};

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
    next_id: AtomicUsize,
}

impl BotManager {
    /// Create a new [`BotManager`].
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
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
        let bot = KernelBot::new(config, bus);
        let join = self.run_bot_decision_loop(bot);
        Ok(BotHandle { id, join })
    }

    /// Run the decision loop for a bot asynchronously.
    pub fn run_bot_decision_loop(&self, bot: KernelBot) -> JoinHandle<BotState> {
        tokio::spawn(async move { bot.run() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bot::AiType;

    #[tokio::test]
    async fn spawns_bot_and_returns_handle() {
        let manager = BotManager::new();
        let bus = Arc::new(EventBus::new());
        let cfg = BotConfig::new("b", AiType::Heuristic);
        let handle = manager.spawn_bot(cfg, bus).expect("spawn");
        assert_eq!(handle.id, 0);
        handle.abort();
    }
}
