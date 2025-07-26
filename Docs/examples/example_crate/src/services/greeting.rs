//! Greeting service trait and default implementation.
use crate::error::Result;

/// Produces greeting messages.
///
/// # Examples
///
/// ```
/// use example_crate::services::{GreetingService, DefaultGreetingService};
/// let svc = DefaultGreetingService;
/// let msg = svc.greeting("Bob").unwrap();
/// assert_eq!(msg, "Hello, Bob!");
/// ```
pub trait GreetingService: Send + Sync {
    /// Generate a greeting for `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// use example_crate::services::{GreetingService, DefaultGreetingService};
    /// let svc = DefaultGreetingService;
    /// assert_eq!(svc.greeting("Bob").unwrap(), "Hello, Bob!");
    /// ```
    ///
    /// # Errors
    ///
    /// Propagates failures from the implementation.
    fn greeting(&self, name: &str) -> Result<String>;
}

/// Simple implementation returning `Hello, name!`.
///
/// # Examples
///
/// ```
/// use example_crate::services::{GreetingService, DefaultGreetingService};
/// let svc = DefaultGreetingService;
/// assert_eq!(svc.greeting("Amy").unwrap(), "Hello, Amy!");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefaultGreetingService;

impl GreetingService for DefaultGreetingService {
    fn greeting(&self, name: &str) -> Result<String> {
        Ok(format!("Hello, {name}!"))
    }
}
