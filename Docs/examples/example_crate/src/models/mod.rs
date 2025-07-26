//! Data models used by services and processors.

use uuid::Uuid;

/// Simple user model.
///
/// # Examples
///
/// ```
/// use example_crate::models::User;
/// use uuid::Uuid;
/// let u = User::new(Uuid::nil(), "bob");
/// assert_eq!(u.name, "bob");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    /// Unique identifier.
    pub id: Uuid,
    /// Display name.
    pub name: String,
}

impl User {
    /// Create a new [`User`].
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::models::User;
    /// use uuid::Uuid;
    /// let u = User::new(Uuid::nil(), "alice");
    /// assert_eq!(u.id, Uuid::nil());
    /// ```
    pub fn new(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
