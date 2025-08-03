//! Core bot crate coordinating decision making.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Action handling for executing decisions.
pub mod action;
/// Built-in AI implementations.
pub mod ai;
/// Bot kernel and related types.
pub mod bot;
/// Common error types for the bot crate.
pub mod error;
/// Perception system converting snapshots into observations.
pub mod perception;

pub use action::{Action, ActionExecutor, ActionResult};
pub use ai::{AiType, HeuristicAI, PlanningAI, ReactiveAI, SwitchingAI};
pub use bot::{Bot, BotConfig, BotState, DecisionMaker};
pub use error::BotError;
pub use perception::{BotMemory, Observation, PerceptionSystem};

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
