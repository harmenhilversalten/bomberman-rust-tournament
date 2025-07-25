//! Collection of trait definitions used throughout the example crate.

/// Abstraction for sending messages.
pub trait Messenger {
    /// Send a message.
    fn send_message(&self, msg: &str);
}
