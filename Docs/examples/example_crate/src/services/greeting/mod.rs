//! Types for creating and using a [`GreetingService`].

mod service;
mod builder;

pub use builder::GreetingServiceBuilder;
pub use service::GreetingService;
