//! Greeter processor combining services.
use crate::error::Result;
use crate::providers::UuidProvider;
use crate::services::GreetingService;

/// Processor combining a [`GreetingService`] and [`UuidProvider`].
pub struct GreeterProcessor<'a> {
    greeting: &'a dyn GreetingService,
    uuid: &'a dyn UuidProvider,
}

impl<'a> GreeterProcessor<'a> {
    /// Create a new processor.
    pub fn new(greeting: &'a dyn GreetingService, uuid: &'a dyn UuidProvider) -> Self {
        Self { greeting, uuid }
    }

    /// Produce a greeting with a UUID suffix.
    pub fn process(&self, name: &str) -> Result<String> {
        let msg = self.greeting.greeting(name)?;
        let id = self.uuid.uuid()?;
        Ok(format!("{msg} ({id})"))
    }
}
