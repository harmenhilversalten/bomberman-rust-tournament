//! High-level processor combining services.
//!
//! ```
//! use example_crate::{container::AppModule, processors::HelloWorldProcessor};
//! use shaku::HasComponent;
//! let module = AppModule::builder().build();
//! let greeter = module.resolve();
//! let name_provider = module.resolve();
//! let processor = HelloWorldProcessor::new(greeter, name_provider);
//! let greeting = processor.run().unwrap();
//! assert_eq!(greeting.message, "Hello, World!");
//! ```

use crate::{error::Result, models::Greeting, providers::NameProvider, services::Greeter};
use std::sync::Arc;
#[cfg(feature = "logging")]
use tracing::info;

/// Processor that generates a greeting using injected components.
pub struct HelloWorldProcessor {
    #[doc(hidden)]
    pub(crate) greeter: Arc<dyn Greeter>,
    #[doc(hidden)]
    pub(crate) name_provider: Arc<dyn NameProvider>,
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
    pub fn run(&self) -> Result<Greeting> {
        let name = self.name_provider.name()?;
        let msg = self.greeter.greet(&name)?;
        #[cfg(feature = "logging")]
        info!(target: "processor", %msg, "generated greeting");
        Ok(Greeting::new(msg))
    }
}
