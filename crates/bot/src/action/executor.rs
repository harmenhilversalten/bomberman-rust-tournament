//! Executes actions against a mutable game state.

use super::{Action, ActionResult};

/// Applies [`Action`]s to a mutable integer state for testing.
#[derive(Debug, Default)]
pub struct ActionExecutor;

impl ActionExecutor {
    /// Create a new [`ActionExecutor`].
    pub fn new() -> Self {
        Self
    }

    /// Execute the given action, mutating `state` when successful.
    pub fn execute(&self, state: &mut i32, action: Action) -> ActionResult {
        match action {
            Action::Move(delta) if delta >= 0 => {
                *state += delta;
                ActionResult::Success
            }
            Action::Move(_) => ActionResult::Failure("negative move"),
            Action::Idle => ActionResult::Success,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_action_increments_state() {
        let exec = ActionExecutor::new();
        let mut state = 0;
        let result = exec.execute(&mut state, Action::Move(2));
        assert_eq!(state, 2);
        assert_eq!(result, ActionResult::Success);
    }

    #[test]
    fn negative_move_fails() {
        let exec = ActionExecutor::new();
        let mut state = 0;
        let result = exec.execute(&mut state, Action::Move(-1));
        assert_eq!(state, 0);
        assert_eq!(result, ActionResult::Failure("negative move"));
    }
}
