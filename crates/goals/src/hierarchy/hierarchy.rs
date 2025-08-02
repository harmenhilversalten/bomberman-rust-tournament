//! Goal hierarchy implementation.

use std::collections::{HashMap, HashSet};

use crate::goal::{Goal, GoalType};

use super::dependency::GoalDependency;

/// Node in the hierarchy containing a goal and its dependencies.
pub struct GoalNode {
    /// Goal to execute.
    pub goal: Box<dyn Goal>,
    /// Dependencies required before execution.
    pub dependency: GoalDependency,
}

/// Manages hierarchical goal dependencies.
#[derive(Default)]
pub struct GoalHierarchy {
    nodes: HashMap<GoalType, GoalNode>,
    completed: HashSet<GoalType>,
}

impl GoalHierarchy {
    /// Adds a goal with its dependency information.
    pub fn add_goal(&mut self, goal: Box<dyn Goal>, dependency: GoalDependency) {
        let goal_type = goal.get_goal_type();
        self.nodes.insert(goal_type, GoalNode { goal, dependency });
    }

    /// Marks a goal type as completed.
    pub fn mark_completed(&mut self, goal_type: GoalType) {
        self.completed.insert(goal_type);
    }

    /// Returns goals ready for execution (dependencies satisfied).
    pub fn next_ready(&self) -> Vec<&GoalNode> {
        self.nodes
            .values()
            .filter(|node| {
                let gtype = node.goal.get_goal_type();
                !self.completed.contains(&gtype) && node.dependency.is_satisfied(&self.completed)
            })
            .collect()
    }
}
