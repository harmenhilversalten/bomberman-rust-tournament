//! Builder for [`GreetingService`](super::greeting_service::GreetingService).

use crate::helpers::GreetingFormatter;
use crate::traits::Greeter;
use super::greeting_service::GreetingService;

/// Builder for [`GreetingService`].
///
/// This allows optional configuration of a [`GreetingFormatter`].
pub struct GreetingServiceBuilder<G: Greeter> {
    greeter: G,
    formatter: Option<Box<dyn GreetingFormatter>>, 
}

impl<G: Greeter> GreetingServiceBuilder<G> {
    /// Create a new builder backed by the provided `greeter`.
    pub fn new(greeter: G) -> Self {
        Self { greeter, formatter: None }
    }

    /// Set a formatter to be used by the service.
    pub fn with_formatter(mut self, formatter: Box<dyn GreetingFormatter>) -> Self {
        self.formatter = Some(formatter);
        self
    }

    /// Construct the [`GreetingService`].
    pub fn build(self) -> GreetingService<G> {
        GreetingService::from_parts(self.greeter, self.formatter)
    }
}

