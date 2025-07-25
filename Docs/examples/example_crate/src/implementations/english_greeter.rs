//! Implementation of [`crate::traits::Greeter`] that outputs messages in English.

use crate::traits::Greeter;

/// Greeter implementation that produces English messages.
pub struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}
