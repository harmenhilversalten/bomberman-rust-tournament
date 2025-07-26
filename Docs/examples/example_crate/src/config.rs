//! Shared configuration.
/// Shared configuration for the application.
///
/// # Examples
///
/// ```
/// use example_crate::config::Config;
/// let c = Config::new("svc");
/// assert_eq!(c.service_name, "svc");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Config {
    /// Arbitrary service name.
    pub service_name: String,
}

impl Config {
    /// Create a new config with `service_name`.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::config::Config;
    /// let cfg = Config::new("demo");
    /// assert_eq!(cfg.service_name, "demo");
    /// ```
    #[must_use]
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}
