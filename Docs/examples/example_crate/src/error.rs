//! Error types for the crate.
use thiserror::Error;

/// Crate-wide error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Generic failure.
    #[error("generic error: {0}")]
    Generic(String),
}
/// Result type for the crate.
pub type Result<T> = std::result::Result<T, Error>;
