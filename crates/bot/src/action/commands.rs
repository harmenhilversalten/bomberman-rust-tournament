//! Definitions of possible bot actions.

use bombs::{Direction, Position};

/// Action set available to bots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Move in a direction.
    Move(Direction),
    /// Place a bomb at a position.
    PlaceBomb {
        /// Grid position where the bomb should be placed.
        position: Position,
    },
    /// Do nothing this tick.
    Wait,
}
