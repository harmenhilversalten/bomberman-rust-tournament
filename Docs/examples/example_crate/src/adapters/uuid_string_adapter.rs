//! UUID string adapter example.
//! Converts [`UuidProvider`] output into `String` values.

use crate::error::Result;
use crate::providers::UuidProvider;

/// Adapter returning UUIDs as strings.
///
/// # Examples
///
/// ```
/// use example_crate::adapters::{DefaultUuidStringAdapter, UuidStringAdapter};
/// use example_crate::providers::DefaultUuidProvider;
/// let adapter = DefaultUuidStringAdapter::new(&DefaultUuidProvider);
/// let _uuid = adapter.uuid_string().unwrap();
/// ```
pub trait UuidStringAdapter: Send + Sync {
    /// Generate a UUID string.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::adapters::{DefaultUuidStringAdapter, UuidStringAdapter};
    /// use example_crate::providers::DefaultUuidProvider;
    /// let adapter = DefaultUuidStringAdapter::new(&DefaultUuidProvider);
    /// let _ = adapter.uuid_string().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error from the underlying [`UuidProvider`].
    fn uuid_string(&self) -> Result<String>;
}

/// Default adapter using a [`UuidProvider`].
///
/// # Examples
///
/// ```
/// use example_crate::adapters::DefaultUuidStringAdapter;
/// use example_crate::providers::DefaultUuidProvider;
/// let adapter = DefaultUuidStringAdapter::new(&DefaultUuidProvider);
/// ```
pub struct DefaultUuidStringAdapter<'a> {
    provider: &'a dyn UuidProvider,
}

impl<'a> DefaultUuidStringAdapter<'a> {
    /// Create a new adapter wrapping the given provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::adapters::DefaultUuidStringAdapter;
    /// use example_crate::providers::DefaultUuidProvider;
    /// let _adapter = DefaultUuidStringAdapter::new(&DefaultUuidProvider);
    /// ```
    #[must_use]
    pub fn new(provider: &'a dyn UuidProvider) -> Self {
        Self { provider }
    }
}

impl UuidStringAdapter for DefaultUuidStringAdapter<'_> {
    fn uuid_string(&self) -> Result<String> {
        self.provider.uuid().map(|id| id.to_string())
    }
}
