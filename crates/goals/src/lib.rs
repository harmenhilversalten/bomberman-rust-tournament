//! Goal management crate providing goal definitions, planning, execution, and hierarchy.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Goal execution utilities.
pub mod executor;
/// Goal definitions and utilities.
pub mod goal;
/// Goal hierarchy management.
pub mod hierarchy;
/// Goal planning utilities.
pub mod planner;

pub use executor::{GoalExecutor, ProgressMonitor};
pub use goal::{Action, AvoidDangerGoal, BotId, CollectPowerUpGoal, Goal, GoalError, GoalType};
pub use hierarchy::{GoalDependency, GoalHierarchy, GoalNode};
pub use planner::{GoalPlanner, PlanningStrategy};
