//! Utilities for formatting greeting messages.

/// Trait defining a formatter for greeting messages.
pub trait GreetingFormatter {
    /// Format the provided `msg` and return the resulting `String`.
    fn format(&self, msg: &str) -> String;
}

/// Formatter that appends a smiley emoji to the message.
pub struct EmojiFormatter;

impl GreetingFormatter for EmojiFormatter {
    fn format(&self, msg: &str) -> String {
        format!("{msg} \u{1F60A}")
    }
}
