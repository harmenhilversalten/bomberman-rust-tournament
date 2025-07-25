//! Service implementations that make use of [`Greeter`](crate::traits::Greeter).

use crate::helpers::GreetingFormatter;
use crate::traits::Greeter;

use super::greeting_service_builder::GreetingServiceBuilder;

/// Service that delegates greeting creation to a [`Greeter`].
pub struct GreetingService<G: Greeter> {
    greeter: G,
    formatter: Option<Box<dyn GreetingFormatter>>,
}

impl<G: Greeter> GreetingService<G> {
    pub(crate) fn from_parts(
        greeter: G,
        formatter: Option<Box<dyn GreetingFormatter>>, 
    ) -> Self {
        Self { greeter, formatter }
    }

    /// Create a new service backed by the provided `greeter`.
    pub fn new(greeter: G) -> Self {
        GreetingServiceBuilder::new(greeter).build()
    }

    /// Create a new service backed by the provided `greeter` and `formatter`.
    pub fn with_formatter(greeter: G, formatter: Box<dyn GreetingFormatter>) -> Self {
        GreetingServiceBuilder::new(greeter)
            .with_formatter(formatter)
            .build()
    }

    /// Generate a greeting for `name`.
    pub fn send_greeting(&self, name: &str) -> String {
        let msg = self.greeter.greet(name);
        match &self.formatter {
            Some(f) => f.format(&msg),
            None => msg,
        }
    }
}
