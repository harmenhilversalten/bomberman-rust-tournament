//! Dependency injection container.
use crate::config::Config;
use crate::providers::{DefaultUuidProvider, UuidProvider};
use crate::services::{DefaultGreetingService, GreetingService};

/// Lightweight dependency injection container.
///
/// # Examples
///
/// ```
/// use example_crate::{config::Config, container::Container};
/// let c = Container::new(Config::new("svc"));
/// assert_eq!(c.config().service_name, "svc");
/// ```
pub struct Container {
    config: Config,
    greeting: Box<dyn GreetingService>,
    uuid: Box<dyn UuidProvider>,
}

impl Container {
    /// Build a new container with default implementations.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::{config::Config, container::Container};
    /// let _c = Container::new(Config::new("svc"));
    /// ```
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            config,
            greeting: Box::new(DefaultGreetingService),
            uuid: Box::new(DefaultUuidProvider),
        }
    }

    /// Get the configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::{config::Config, container::Container};
    /// let c = Container::new(Config::new("svc"));
    /// assert_eq!(c.config().service_name, "svc");
    /// ```
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Access the [`GreeterProcessor`].
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::{config::Config, container::Container};
    /// let c = Container::new(Config::new("svc"));
    /// let processor = c.greeter_processor();
    /// let _ = processor.process("Bob");
    /// ```
    #[must_use]
    pub fn greeter_processor(&self) -> super::processors::GreeterProcessor {
        super::processors::GreeterProcessor::new(self.greeting.as_ref(), self.uuid.as_ref())
    }

    /// Access the [`DefaultUuidStringAdapter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::{adapters::UuidStringAdapter, config::Config, container::Container};
    /// let c = Container::new(Config::new("svc"));
    /// let adapter = c.uuid_string_adapter();
    /// let _id = adapter.uuid_string().unwrap();
    /// ```
    #[must_use]
    pub fn uuid_string_adapter(&self) -> super::adapters::DefaultUuidStringAdapter {
        super::adapters::DefaultUuidStringAdapter::new(self.uuid.as_ref())
    }
}
