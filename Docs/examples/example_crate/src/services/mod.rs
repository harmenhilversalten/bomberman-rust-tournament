//! Service layer built on top of [`Greeter`](crate::traits::Greeter) implementations.

pub mod greeting_service;
pub mod greeting_service_builder;

pub use greeting_service::GreetingService;
pub use greeting_service_builder::GreetingServiceBuilder;
