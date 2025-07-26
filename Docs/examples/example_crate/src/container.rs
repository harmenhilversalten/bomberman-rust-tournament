//! Dependency injection container.
use crate::config::Config;
use crate::providers::{DefaultUuidProvider, UuidProvider};
use crate::services::{DefaultGreetingService, GreetingService};

/// Lightweight dependency injection container.
pub struct Container {
    config: Config,
    greeting: Box<dyn GreetingService>,
    uuid: Box<dyn UuidProvider>,
}

impl Container {
    /// Build a new container with default implementations.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            greeting: Box::new(DefaultGreetingService),
            uuid: Box::new(DefaultUuidProvider),
        }
    }

    /// Get the configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Access the [`GreeterProcessor`].
    pub fn greeter_processor(&self) -> super::processors::GreeterProcessor {
        super::processors::GreeterProcessor::new(self.greeting.as_ref(), self.uuid.as_ref())
    }
}
