//! UUID provider trait and default implementation.
use crate::error::Result;
use uuid::Uuid;

/// Supplies UUIDs.
pub trait UuidProvider: Send + Sync {
    /// Generate a new UUID.
    fn uuid(&self) -> Result<Uuid>;
}

/// Default provider using `Uuid::new_v4`.
pub struct DefaultUuidProvider;

impl UuidProvider for DefaultUuidProvider {
    fn uuid(&self) -> Result<Uuid> {
        Ok(Uuid::new_v4())
    }
}
