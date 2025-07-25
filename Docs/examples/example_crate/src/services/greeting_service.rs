//! Service implementations that make use of [`Greeter`](crate::traits::Greeter).

use crate::traits::Greeter;

/// Service that delegates greeting creation to a [`Greeter`].
pub struct GreetingService<G: Greeter> {
    greeter: G,
}

impl<G: Greeter> GreetingService<G> {
    /// Create a new service backed by the provided `greeter`.
    pub fn new(greeter: G) -> Self {
        Self { greeter }
    }

    /// Generate a greeting for `name`.
    pub fn send_greeting(&self, name: &str) -> String {
        self.greeter.greet(name)
    }
}
