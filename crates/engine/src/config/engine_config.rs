use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use super::{ConfigError, GameRules};

/// Configuration for the game engine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineConfig {
    /// Width of the game grid.
    pub width: usize,
    /// Height of the game grid.
    pub height: usize,
    /// Target ticks per second.
    pub tick_rate: u32,
    /// Game rules applied to the simulation.
    pub rules: GameRules,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            width: 13,
            height: 11,
            tick_rate: 60,
            rules: GameRules::default(),
        }
    }
}

impl EngineConfig {
    /// Load configuration from a JSON file.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(ConfigError::Json)
    }

    /// Attempt to load configuration from a file, falling back to defaults.
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::from_path(path).unwrap_or_default()
    }

    /// Validate the engine configuration.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.width == 0 || self.height == 0 {
            return Err(ConfigError::Invalid(
                "grid dimensions must be greater than zero".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_config_from_file() {
        let path =
            std::env::temp_dir().join(format!("engine_config_test_{}.json", std::process::id()));
        let json = r#"{
            "width": 5,
            "height": 6,
            "tick_rate": 30,
            "rules": {"max_players": 2, "bomb_timer": 5, "starting_lives": 1}
        }"#;
        fs::write(&path, json).unwrap();
        let cfg = EngineConfig::from_path(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(cfg.width, 5);
        assert_eq!(cfg.rules.bomb_timer, 5);
        assert_eq!(cfg.tick_rate, 30);
    }
}
