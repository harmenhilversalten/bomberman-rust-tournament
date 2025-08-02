//! Error types for the RL crate.
use thiserror::Error;

/// Errors that can occur within the RL crate.
#[derive(Debug, Error)]
pub enum RLError {
    /// Wrapper around errors originating from the `tch` crate.
    #[error(transparent)]
    Tch(#[from] tch::TchError),
    /// Wrapper around I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
