//! Error types for the RL crate.
use thiserror::Error;

/// Errors that can occur in reinforcement learning operations.
#[derive(Debug, Error)]
pub enum RLError {
    /// An underlying I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// A model specific failure.
    #[error("model error: {0}")]
    Model(String),
}
