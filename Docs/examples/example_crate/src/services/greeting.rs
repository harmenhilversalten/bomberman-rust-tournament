//! Greeting service trait and default implementation.
use crate::error::Result;

/// Produces greeting messages.
pub trait GreetingService: Send + Sync {
    /// Generate a greeting for `name`.
    fn greeting(&self, name: &str) -> Result<String>;
}

/// Simple implementation returning `Hello, name!`.
pub struct DefaultGreetingService;

impl GreetingService for DefaultGreetingService {
    fn greeting(&self, name: &str) -> Result<String> {
        Ok(format!("Hello, {name}!"))
    }
}
