//! Core bot crate coordinating decision making.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Bot kernel and related types.
pub mod bot;

pub use bot::{Bot, BotConfig, BotState, DecisionMaker};

/// Initializes the crate and returns a greeting.
pub fn init() -> &'static str {
    "initialized"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_returns_initialized() {
        assert_eq!(init(), "initialized");
    }
}
