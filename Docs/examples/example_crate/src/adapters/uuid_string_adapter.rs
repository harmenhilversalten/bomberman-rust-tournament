//! UUID string adapter example.
//! Converts [`UuidProvider`] output into `String` values.

use crate::error::Result;
use crate::providers::UuidProvider;

/// Adapter returning UUIDs as strings.
pub trait UuidStringAdapter: Send + Sync {
    /// Generate a UUID string.
    fn uuid_string(&self) -> Result<String>;
}

/// Default adapter using a [`UuidProvider`].
pub struct DefaultUuidStringAdapter<'a> {
    provider: &'a dyn UuidProvider,
}

impl<'a> DefaultUuidStringAdapter<'a> {
    /// Create a new adapter wrapping the given provider.
    pub fn new(provider: &'a dyn UuidProvider) -> Self {
        Self { provider }
    }
}

impl<'a> UuidStringAdapter for DefaultUuidStringAdapter<'a> {
    fn uuid_string(&self) -> Result<String> {
        self.provider.uuid().map(|id| id.to_string())
    }
}
