//! Simple greeting service without builder.

use crate::traits::Greeter;

/// Service that directly forwards greetings using a [`Greeter`].
pub struct SimpleGreetingService<G: Greeter> {
    greeter: G,
}

impl<G: Greeter> SimpleGreetingService<G> {
    /// Create a new service backed by `greeter`.
    pub fn new(greeter: G) -> Self {
        Self { greeter }
    }

    /// Produce a greeting for `name` using the underlying `greeter`.
    pub fn send_greeting(&self, name: &str) -> String {
        self.greeter.greet(name)
    }
}
