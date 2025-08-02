//! Goal execution coordinating planner output and progress monitoring.

use state::GameState;

use crate::goal::{Action, BotId, GoalError};
use crate::planner::GoalPlanner;

use super::monitor::ProgressMonitor;

/// Executes the active goal from a planner while tracking progress.
pub struct GoalExecutor {
    monitor: ProgressMonitor,
}

impl GoalExecutor {
    /// Creates a new executor with the given progress monitor.
    pub fn new(monitor: ProgressMonitor) -> Self {
        Self { monitor }
    }

    /// Runs one tick of the active goal, returning actions to perform.
    pub fn execute(
        &mut self,
        planner: &mut GoalPlanner,
        state: &GameState,
        bot_id: BotId,
    ) -> Result<Vec<Action>, GoalError> {
        let actions = planner.execute_active_goal(state, bot_id)?;
        if let Some(ref active) = planner.active_goal {
            self.monitor.update(active.progress);
        }
        Ok(actions)
    }

    /// Whether the executor considers the active goal stalled.
    pub fn is_stalled(&self) -> bool {
        self.monitor.is_stalled()
    }
}
