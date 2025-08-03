use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{engine_config::EngineConfig, tournament_config::TournamentConfig};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml parse error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid configuration: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusConfig {
    pub buffer_size: usize,
    pub max_subscribers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    pub name: String,
    pub ai_type: String,
    pub rl_mode: bool,
    pub rl_model_path: Option<String>,
    pub decision_timeout_ms: u64,
}

impl BotConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.name.trim().is_empty() {
            return Err(ConfigError::Invalid("bot name cannot be empty".into()));
        }
        if self.rl_mode && self.rl_model_path.is_none() {
            return Err(ConfigError::Invalid(
                "rl_mode enabled but rl_model_path missing".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLConfig {
    pub model_path: String,
}

impl RLConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !Path::new(&self.model_path).exists() {
            return Err(ConfigError::Invalid(format!(
                "rl model path not found: {}",
                self.model_path
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BombConfig {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConfig {
    pub engine: EngineConfig,
    pub event_bus: EventBusConfig,
    pub bots: Vec<BotConfig>,
    pub tournament: Option<TournamentConfig>,
    pub ai: AIConfig,
    pub rl: Option<RLConfig>,
    pub bombs: BombConfig,
    pub logging: LoggingConfig,
}

impl UnifiedConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.engine.validate()?;
        for bot in &self.bots {
            bot.validate()?;
        }
        if let Some(rl) = &self.rl {
            rl.validate()?;
        }
        self.validate_consistency()
    }

    fn validate_consistency(&self) -> Result<(), ConfigError> {
        if self.bots.len() > self.engine.rules.max_players as usize {
            return Err(ConfigError::Invalid(
                "number of bots exceeds engine rules".into(),
            ));
        }
        Ok(())
    }

    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: Self = if path.ends_with(".toml") {
            toml::from_str(&content)?
        } else {
            serde_json::from_str(&content)?
        };
        config.validate()?;
        Ok(config)
    }

    pub fn with_env_overrides(mut self) -> Result<Self, ConfigError> {
        if let Ok(level) = std::env::var("BOMBER_LOG_LEVEL") {
            self.logging.level = level;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn load_and_validate_from_json() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", r#"{
            "engine":{"width":5,"height":5,"tick_rate":60,"rules":{"max_players":4,"bomb_timer":3,"starting_lives":3}},
            "event_bus":{"buffer_size":10,"max_subscribers":10},
            "bots":[{"name":"b1","ai_type":"Heuristic","rl_mode":false,"rl_model_path":null,"decision_timeout_ms":5}],
            "tournament":null,
            "ai":{},
            "rl":null,
            "bombs":{},
            "logging":{"level":"info"}
        }"#).unwrap();
        let path = file.into_temp_path();
        let cfg = UnifiedConfig::from_file(path.to_str().unwrap()).unwrap();
        assert_eq!(cfg.bots.len(), 1);
    }

    #[test]
    fn validate_rejects_missing_rl_model() {
        let cfg = UnifiedConfig {
            engine: EngineConfig::default(),
            event_bus: EventBusConfig {
                buffer_size: 1,
                max_subscribers: 1,
            },
            bots: vec![],
            tournament: None,
            ai: AIConfig::default(),
            rl: Some(RLConfig {
                model_path: "missing".into(),
            }),
            bombs: BombConfig::default(),
            logging: LoggingConfig {
                level: "info".into(),
            },
        };
        assert!(cfg.validate().is_err());
    }
}
