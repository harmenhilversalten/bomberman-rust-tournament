use std::{error::Error, fmt, fs, path::Path};

use serde::{Deserialize, Serialize};

use super::GameRules;

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
        let data = fs::read_to_string(path).map_err(ConfigError::Io)?;
        serde_json::from_str(&data).map_err(ConfigError::Parse)
    }

    /// Attempt to load configuration from a file, falling back to defaults.
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::from_path(path).unwrap_or_default()
    }
}

/// Errors that may occur while loading configuration.
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Parse(e) => Some(e),
        }
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
