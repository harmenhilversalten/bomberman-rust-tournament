//! Goal management crate providing goal definitions, planning, execution, and hierarchy.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Goal execution utilities.
pub mod executor;
/// Goal definitions and utilities.
pub mod goal;
/// Goal hierarchy management.
pub mod hierarchy;
/// Goal generation utilities.
pub mod manager;
/// Goal planning utilities.
pub mod planner;
/// Goal scoring utilities.
pub mod scoring;

pub use executor::{GoalExecutor, ProgressMonitor};
pub use goal::{Action, AvoidDangerGoal, BotId, CollectPowerUpGoal, Goal, GoalError, GoalType};
pub use hierarchy::{GoalDependency, GoalHierarchy, GoalNode};
pub use manager::{GoalGenerator, GoalManager};
pub use planner::{GoalPlanner, PlanningStrategy};
pub use scoring::{GoalScorer, StateEvaluator};
