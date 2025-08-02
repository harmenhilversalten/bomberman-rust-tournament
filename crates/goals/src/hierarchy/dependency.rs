//! Structures describing goal dependencies.

use std::collections::HashSet;

use crate::goal::GoalType;

/// Describes dependencies that must be completed before a goal can run.
#[derive(Debug, Default, Clone)]
pub struct GoalDependency {
    prerequisites: HashSet<GoalType>,
}

impl GoalDependency {
    /// Creates a dependency from an iterator of goal types.
    pub fn with<I: IntoIterator<Item = GoalType>>(deps: I) -> Self {
        Self {
            prerequisites: deps.into_iter().collect(),
        }
    }

    /// Returns true if all prerequisites are satisfied.
    pub fn is_satisfied(&self, completed: &HashSet<GoalType>) -> bool {
        self.prerequisites.is_subset(completed)
    }
}
