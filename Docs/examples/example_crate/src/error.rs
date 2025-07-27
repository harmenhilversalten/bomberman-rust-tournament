//! Crate-wide error definitions.

use thiserror::Error;

/// Result type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A generic error with a message.
    #[error("{0}")]
    Generic(String),
}
