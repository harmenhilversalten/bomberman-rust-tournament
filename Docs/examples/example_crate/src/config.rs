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
//!
//! ```
//! # #[cfg(feature = "serde")] {
//! use example_crate::config::Config;
//! let cfg = Config { prefix: "Hey".into() };
//! let s = serde_json::to_string(&cfg).unwrap();
//! let back: Config = serde_json::from_str(&s).unwrap();
//! assert_eq!(back.prefix, "Hey");
//! # }
//! ```

use crate::error::{Error, Result};
use std::env;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Application configuration.
pub struct Config {
    /// Prefix used for greetings.
    pub prefix: String,
}

impl Config {
    /// Validate the provided `prefix` string.
    ///
    /// ```
    /// use example_crate::config::Config;
    /// assert!(Config::validate_prefix("Hi").is_ok());
    /// assert!(Config::validate_prefix("").is_err());
    /// ```
    pub fn validate_prefix(prefix: &str) -> Result<()> {
        const MAX_PREFIX_LEN: usize = 32;
        if prefix.trim().is_empty() {
            Err(Error::Config("GREETING_PREFIX is empty".into()))
        } else if prefix.len() > MAX_PREFIX_LEN {
            Err(Error::Config(format!(
                "GREETING_PREFIX must be at most {MAX_PREFIX_LEN} characters"
            )))
        } else {
            Ok(())
        }
    }

    /// Load configuration from environment variables.
    ///
    /// # Errors
    /// Returns an [`Error::Config`] when the prefix is invalid.
    ///
    /// ```
    /// use example_crate::config::Config;
    /// std::env::set_var("GREETING_PREFIX", "Hi");
    /// let cfg = Config::load().unwrap();
    /// assert_eq!(cfg.prefix, "Hi");
    /// ```
    pub fn load() -> Result<Self> {
        let prefix = env::var("GREETING_PREFIX").unwrap_or_else(|_| "Hello".to_owned());
        Self::validate_prefix(&prefix)?;
        Ok(Self { prefix })
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

    #[test]
    fn validate_prefix_rejects_bad_values() {
        assert!(Config::validate_prefix("").is_err());
        let long = "x".repeat(40);
        assert!(Config::validate_prefix(&long).is_err());
    }
}
