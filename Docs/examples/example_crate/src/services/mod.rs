//! Service layer built on top of [`Greeter`](crate::traits::Greeter) implementations.

/// Types related to greeting messages.
pub mod greeting;

pub use greeting::{GreetingService, GreetingServiceBuilder};
