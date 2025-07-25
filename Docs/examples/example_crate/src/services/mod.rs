//! Service layer built on top of [`Greeter`](crate::traits::Greeter) implementations.

pub mod greeting_service;
pub mod builders {
    //! Builder types for services.
    pub mod greeting_service_builder;

    pub use greeting_service_builder::GreetingServiceBuilder;
}

pub use greeting_service::GreetingService;
pub use builders::GreetingServiceBuilder;
