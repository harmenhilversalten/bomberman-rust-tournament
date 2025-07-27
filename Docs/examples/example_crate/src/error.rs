//! Crate-wide error definitions.
//!
//! ```
//! use example_crate::error::{Error, Result};
//! fn always_fails() -> Result<()> { Err(Error::Generic("oops".into())) }
//! assert!(always_fails().is_err());
//! ```

use thiserror::Error;

/// Result type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A generic error with a message.
    #[error("{0}")]
    Generic(String),
    /// Configuration related error.
    #[error("config error: {0}")]
    Config(String),
}
