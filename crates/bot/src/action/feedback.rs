//! Result of executing an [`Action`].

/// Outcome of attempting to apply an action to the game state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    /// The action succeeded.
    Success,
    /// The action failed with a reason.
    Failure(&'static str),
}
