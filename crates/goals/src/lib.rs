//! Goal management crate providing goal definitions and planning.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Goal definitions and utilities.
pub mod goal;
/// Goal planning utilities.
pub mod planner;

pub use goal::{Action, AvoidDangerGoal, BotId, CollectPowerUpGoal, Goal, GoalError, GoalType};
pub use planner::{GoalPlanner, PlanningStrategy};
