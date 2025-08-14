use serde::{Deserialize, Serialize};

/// Common types and utilities shared across multiple crates.

pub mod diagnostics;
pub mod logging;

/// Represents a cardinal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Up direction.
    Up,
    /// Down direction.
    Down,
    /// Left direction.
    Left,
    /// Right direction.
    Right,
}
