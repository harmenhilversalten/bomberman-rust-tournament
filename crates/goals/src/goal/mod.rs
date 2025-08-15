#![allow(clippy::module_inception)]

/// Core goal trait and supporting types.
pub mod goal;
/// Built-in goal implementations.
pub mod goal_types;
/// Attack and destroy goal implementations.
pub mod attack_goal;
/// Priority calculation helpers.
pub mod priority;

pub use goal::{Action, BotId, Goal, GoalError, GoalType};
pub use goal_types::{AvoidDangerGoal, CollectPowerUpGoal};
pub use attack_goal::{AttackEnemyGoal, DestroyBlocksGoal};
pub use priority::weighted_priority;
