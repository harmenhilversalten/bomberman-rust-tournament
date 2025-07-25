//! Trait definitions for the example crate.

/// Defines behavior for generating greeting messages.
pub trait Greeter {
    /// Produce a greeting for the provided `name`.
    fn greet(&self, name: &str) -> String;
}
