//! High-level processor combining services.

use crate::error::Result;
use crate::providers::NameProvider;
use crate::services::Greeter;
use std::sync::Arc;

/// Processor that generates a greeting using injected components.
pub struct HelloWorldProcessor {
    greeter: Arc<dyn Greeter>,
    name_provider: Arc<dyn NameProvider>,
}

impl HelloWorldProcessor {
    /// Create a new processor from `greeter` and `name_provider`.
    #[must_use]
    pub fn new(greeter: Arc<dyn Greeter>, name_provider: Arc<dyn NameProvider>) -> Self {
        Self {
            greeter,
            name_provider,
        }
    }

    /// Produce the greeting message.
    pub fn run(&self) -> Result<String> {
        let name = self.name_provider.name()?;
        self.greeter.greet(&name)
    }
}
