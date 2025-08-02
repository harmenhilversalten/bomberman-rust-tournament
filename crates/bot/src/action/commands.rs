//! Definitions of possible bot actions.

/// Minimal action set used by tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Move by the provided delta.
    Move(i32),
    /// Do nothing this tick.
    Idle,
}
