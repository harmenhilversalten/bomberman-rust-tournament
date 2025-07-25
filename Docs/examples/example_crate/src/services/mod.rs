//! Service layer built on top of [`Greeter`](crate::traits::Greeter) implementations.

pub mod greeting_service;

pub use greeting_service::GreetingService;
