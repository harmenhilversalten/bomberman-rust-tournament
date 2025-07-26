//! UUID provider trait and default implementation.
use crate::error::Result;
use uuid::Uuid;

/// Supplies UUIDs.
///
/// # Examples
///
/// ```
/// use example_crate::providers::{DefaultUuidProvider, UuidProvider};
/// let provider = DefaultUuidProvider;
/// let id = provider.uuid().unwrap();
/// assert_eq!(id.get_version_num(), 4);
/// ```
pub trait UuidProvider: Send + Sync {
    /// Generate a new UUID.
    ///
    /// # Errors
    ///
    /// Propagates errors from the provider implementation.
    fn uuid(&self) -> Result<Uuid>;
}

/// Default provider using `Uuid::new_v4`.
///
/// # Examples
///
/// ```
/// use example_crate::providers::{DefaultUuidProvider, UuidProvider};
/// let provider = DefaultUuidProvider;
/// let _id = provider.uuid().unwrap();
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefaultUuidProvider;

impl UuidProvider for DefaultUuidProvider {
    fn uuid(&self) -> Result<Uuid> {
        Ok(Uuid::new_v4())
    }
}
