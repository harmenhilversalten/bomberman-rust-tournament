//! Data models used by services and processors.

use uuid::Uuid;

/// Simple user model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    /// Unique identifier.
    pub id: Uuid,
    /// Display name.
    pub name: String,
}

impl User {
    /// Create a new [`User`].
    pub fn new(id: Uuid, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}
