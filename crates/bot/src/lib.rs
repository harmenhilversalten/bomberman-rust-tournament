//! Core bot crate coordinating decision making.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Built-in AI implementations.
pub mod ai;
/// Bot kernel and related types.
pub mod bot;

pub use ai::{AiType, HeuristicAI, PlanningAI, ReactiveAI, SwitchingAI};
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
