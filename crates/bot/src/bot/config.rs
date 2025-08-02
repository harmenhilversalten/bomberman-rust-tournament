use std::time::Duration;

use events::events::bot_events::BotId;

use crate::ai::AiType;

/// Errors that may occur when validating a [`BotConfig`].
#[derive(Debug, thiserror::Error)]
pub enum BotConfigError {
    /// The bot name was empty.
    #[error("bot name cannot be empty")]
    EmptyName,
}

/// Configuration options for a [`Bot`].
#[derive(Debug, Clone)]
pub struct BotConfig {
    /// Unique identifier for the bot, assigned by the engine.
    pub id: BotId,
    /// Human readable name of the bot.
    pub name: String,
    /// Selected AI strategy for this bot.
    pub ai_type: AiType,
    /// Maximum allowed time for making a single decision.
    pub decision_timeout: Duration,
}

impl BotConfig {
    /// Create a new configuration with the given name and AI type.
    pub fn new(name: &str, ai_type: AiType) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            ai_type,
            decision_timeout: Duration::from_millis(2),
        }
    }

    /// Validate the configuration returning an error if invalid.
    pub fn validate(&self) -> Result<(), BotConfigError> {
        if self.name.trim().is_empty() {
            Err(BotConfigError::EmptyName)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_validates() {
        let cfg = BotConfig::new("test", AiType::Heuristic);
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn empty_name_is_invalid() {
        let cfg = BotConfig::new("", AiType::Reactive);
        assert!(cfg.validate().is_err());
    }
}
