//! Configuration utilities for the crate.
//!
//! The [`Config`] struct holds runtime configuration loaded from environment
//! variables.
//!
//! ```
//! use example_crate::config::Config;
//! std::env::set_var("GREETING_PREFIX", "Hi");
//! let cfg = Config::load().unwrap();
//! assert_eq!(cfg.prefix, "Hi");
//! ```

use crate::error::Result;
use std::env;

/// Application configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Prefix used for greetings.
    pub prefix: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn load() -> Result<Self> {
        let prefix = env::var("GREETING_PREFIX").unwrap_or_else(|_| "Hello".to_owned());
        if prefix.trim().is_empty() {
            Err(crate::error::Error::Config(
                "GREETING_PREFIX is empty".into(),
            ))
        } else {
            Ok(Self { prefix })
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: "Hello".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn load_uses_env_or_default() {
        std::env::remove_var("GREETING_PREFIX");
        let cfg = Config::load().unwrap();
        assert_eq!(cfg.prefix, "Hello");
        std::env::set_var("GREETING_PREFIX", "Yo");
        let cfg = Config::load().unwrap();
        assert_eq!(cfg.prefix, "Yo");
    }
}
