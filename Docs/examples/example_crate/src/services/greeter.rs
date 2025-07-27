//! Greeting service trait and implementation.
//!
//! ```
//! use example_crate::services::{Greeter, EnglishGreeter};
//! let g = EnglishGreeter;
//! assert_eq!(g.greet("Rust").unwrap(), "Hello, Rust!");
//! ```

use crate::error::Result;
use shaku::{Component, Interface};

/// Generates greeting messages.
pub trait Greeter: Interface {
    /// Return a greeting for `name`.
    fn greet(&self, name: &str) -> Result<String>;
}

/// English implementation of [`Greeter`].
#[derive(Component)]
#[shaku(interface = Greeter)]
pub struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> Result<String> {
        Ok(format!("Hello, {name}!"))
    }
}
