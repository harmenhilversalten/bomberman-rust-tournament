use std::collections::HashMap;

use events::events::bot_events::BotId;

use super::TournamentError;
use crate::config::UnifiedBotConfig as BotConfig;

#[derive(Debug)]
pub struct BotRegistry {
    bots: HashMap<BotId, RegisteredBot>,
    next_id: BotId,
}

impl Default for BotRegistry {
    fn default() -> Self {
        Self {
            bots: HashMap::new(),
            next_id: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisteredBot {
    pub _id: BotId,
    pub _config: BotConfig,
    pub _status: BotStatus,
    pub _connection: Option<BotConnection>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BotStatus {
    Registered,
    Ready,
    Playing,
    Disconnected,
    Banned,
}

#[derive(Debug, Clone)]
pub struct BotConnection;

impl BotRegistry {
    pub fn register_bot(&mut self, config: BotConfig) -> Result<BotId, TournamentError> {
        let id = self.next_id;
        self.next_id += 1;
        let bot = RegisteredBot {
            _id: id,
            _config: config,
            _status: BotStatus::Registered,
            _connection: None,
        };
        self.bots.insert(id, bot);
        Ok(id)
    }

    #[allow(dead_code)]
    pub fn get_ready_bots(&self) -> Vec<&RegisteredBot> {
        self.bots
            .values()
            .filter(|b| matches!(b._status, BotStatus::Registered | BotStatus::Ready))
            .collect()
    }

    pub fn get_bot_ids(&self) -> Vec<BotId> {
        self.bots.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_bots() {
        let mut reg = BotRegistry::default();
        let cfg = BotConfig {
            name: "b1".into(),
            ai_type: "Heuristic".into(),
            rl_mode: false,
            rl_model_path: None,
            decision_timeout_ms: 10,
        };
        let id = reg.register_bot(cfg.clone()).unwrap();
        assert_eq!(id, 0);
        let id2 = reg.register_bot(cfg).unwrap();
        assert_eq!(id2, 1);
        assert_eq!(reg.get_ready_bots().len(), 2);
    }
}
