//! Data models used by services and processors.

/// Example user model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    /// User's name.
    pub name: String,
}

impl User {
    /// Create a new user with `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
