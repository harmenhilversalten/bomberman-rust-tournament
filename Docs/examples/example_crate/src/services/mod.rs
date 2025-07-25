//! Service layer built on top of [`Greeter`](crate::traits::Greeter) implementations.

/// Types related to greeting messages.
pub mod greeting;
mod simple_greeting_service;

pub use greeting::{GreetingService, GreetingServiceBuilder};
pub use simple_greeting_service::SimpleGreetingService;
