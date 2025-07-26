//! Shared configuration.
/// Shared configuration for the application.
#[derive(Clone, Debug)]
pub struct Config {
    /// Arbitrary service name.
    pub service_name: String,
}

impl Config {
    /// Create a new config with `service_name`.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}
