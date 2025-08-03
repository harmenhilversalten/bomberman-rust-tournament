use std::time::Duration;

use events::events::bot_events::BotId;

use crate::ai::AiType;

/// Errors that may occur when validating a [`BotConfig`].
#[derive(Debug, thiserror::Error)]
pub enum BotConfigError {
    /// The bot name was empty.
    #[error("bot name cannot be empty")]
    EmptyName,
    /// RL model path missing when RL mode enabled.
    #[error("rl model path missing when rl_mode is true")]
    MissingModelPath,
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
    /// Enable reinforcement learning mode.
    pub rl_mode: bool,
    /// Optional path to the RL model file.
    pub rl_model_path: Option<String>,
    /// Enable additional reward shaping logic.
    pub rl_reward_shaping: bool,
    /// Exploration rate used by RL policies.
    pub rl_exploration_rate: f32,
}

impl BotConfig {
    /// Create a new configuration with the given name and AI type.
    pub fn new(name: &str, ai_type: AiType) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            ai_type,
            decision_timeout: Duration::from_millis(2),
            rl_mode: false,
            rl_model_path: None,
            rl_reward_shaping: false,
            rl_exploration_rate: 0.0,
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

    /// Validate RL specific configuration options.
    pub fn validate_rl_config(&self) -> Result<(), BotConfigError> {
        if self.rl_mode && self.rl_model_path.is_none() {
            Err(BotConfigError::MissingModelPath)
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

    #[test]
    fn rl_config_requires_model_path() {
        let mut cfg = BotConfig::new("rl", AiType::Heuristic);
        cfg.rl_mode = true;
        assert!(matches!(
            cfg.validate_rl_config(),
            Err(BotConfigError::MissingModelPath)
        ));
        cfg.rl_model_path = Some("model.ot".into());
        assert!(cfg.validate_rl_config().is_ok());
    }
}
