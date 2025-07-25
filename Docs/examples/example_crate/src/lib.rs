#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Example crate demonstrating a basic layered architecture.
//!
//! This crate defines a [`crate::traits::Greeter`] trait along with a simple English
//! implementation and a [`crate::services::GreetingService`] that composes a greeter to
//! produce greeting messages.

/// Concrete implementations of the [`crate::traits::Greeter`] trait.
pub mod implementations;
/// Service layer types built on top of [`crate::traits::Greeter`] implementations.
pub mod services;
/// Miscellaneous helper utilities.
pub mod helpers;
/// Core abstractions used by this crate.
pub mod traits;
