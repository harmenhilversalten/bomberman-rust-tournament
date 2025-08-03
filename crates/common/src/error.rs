use thiserror::Error;

use bot::error::BotError;
use engine::config::ConfigError;
use engine::engine::game_engine::EngineError;
use events::error::EventBusError; // engine config error

pub type Result<T> = std::result::Result<T, BombermanError>;

#[derive(Debug, Error)]
pub enum BombermanError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),
    #[error("Bot error: {0}")]
    Bot(#[from] BotError),
    #[error("Event bus error: {0}")]
    EventBus(#[from] EventBusError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl BombermanError {
    pub fn error_code(&self) -> u32 {
        match self {
            BombermanError::Config(_) => 1000,
            BombermanError::Engine(_) => 2000,
            BombermanError::Bot(_) => 3000,
            BombermanError::EventBus(_) => 4000,
            BombermanError::Io(_) => 5000,
            BombermanError::Serialization(_) => 6000,
            BombermanError::Unknown(_) => 9999,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            BombermanError::EventBus(_) | BombermanError::Bot(BotError::DecisionTimeout { .. })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_and_recoverable() {
        let err = BombermanError::Bot(BotError::DecisionTimeout { timeout_ms: 10 });
        assert_eq!(err.error_code(), 3000);
        assert!(err.is_recoverable());
        let err = BombermanError::Config(ConfigError::Invalid("x".into()));
        assert_eq!(err.error_code(), 1000);
        assert!(!err.is_recoverable());
    }
}
