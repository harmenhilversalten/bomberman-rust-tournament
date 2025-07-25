//! Service implementations that make use of [`crate::traits::Greeter`].

use crate::helpers::GreetingFormatter;
use crate::traits::Greeter;

/// Service that delegates greeting creation to a [`crate::traits::Greeter`].
pub struct GreetingService<G: Greeter> {
    greeter: G,
    formatter: Option<Box<dyn GreetingFormatter>>,
}

impl<G: Greeter> GreetingService<G> {
    /// Create a new service backed by the provided `greeter`.
    pub fn new(greeter: G) -> Self {
        Self {
            greeter,
            formatter: None,
        }
    }

    /// Create a new service backed by the provided `greeter` and `formatter`.
    pub fn with_formatter(
        greeter: G,
        formatter: Box<dyn GreetingFormatter>,
    ) -> Self {
        Self {
            greeter,
            formatter: Some(formatter),
        }
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
