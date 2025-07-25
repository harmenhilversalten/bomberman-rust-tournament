//! Service layer built on top of [`crate::traits::Greeter`] implementations.

pub mod greeting_service;

pub use greeting_service::GreetingService;
