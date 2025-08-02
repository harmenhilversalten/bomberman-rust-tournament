#![allow(clippy::module_inception)]
//! Goal hierarchy and dependency management.

/// Dependency tracking structures.
pub mod dependency;
/// Hierarchy implementation.
pub mod hierarchy;

pub use dependency::GoalDependency;
pub use hierarchy::{GoalHierarchy, GoalNode};
