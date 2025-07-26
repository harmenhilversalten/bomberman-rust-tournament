//! Error types for the crate.
use thiserror::Error;

/// Crate-wide error type.
///
/// # Examples
///
/// ```
/// use example_crate::error::{Error, Result};
/// fn fail() -> Result<()> {
///     Err(Error::Generic("oops".into()))
/// }
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
pub enum Error {
    /// Generic failure.
    #[error("generic error: {0}")]
    Generic(String),
}
/// Result type for the crate.
///
/// # Examples
///
/// ```
/// use example_crate::error::{Error, Result};
/// fn fallible() -> Result<()> {
///     Err(Error::Generic("boom".into()))
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;
