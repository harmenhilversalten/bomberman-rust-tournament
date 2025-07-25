#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Example crate demonstrating a basic layered architecture.
//!
//! This crate defines a [`Greeter`] trait along with a simple English
//! implementation and a [`GreetingService`] that composes a greeter to
//! produce greeting messages.

/// Concrete implementations of the [`Greeter`] trait.
pub mod implementations;
/// Service layer types built on top of [`Greeter`] implementations.
pub mod services;
/// Core abstractions used by this crate.
pub mod traits;
