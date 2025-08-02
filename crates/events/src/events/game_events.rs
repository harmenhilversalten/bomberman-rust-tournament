//! Core game events.

/// Identifier for entities within the game.
pub type EntityId = usize;
/// Grid position represented as `(x, y)` coordinates.
pub type Position = (u16, u16);
/// Identifier for bombs.
pub type BombId = usize;

use serde::{Deserialize, Serialize};

/// Events emitted by the game engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    /// An entity moved to a new position.
    EntityMoved {
        /// Entity identifier.
        entity_id: EntityId,
        /// Previous grid position.
        old_position: Position,
        /// New grid position.
        new_position: Position,
    },
    /// A bomb was placed on the grid.
    BombPlaced {
        /// Identifier of the bomb owner.
        entity_id: EntityId,
        /// Unique bomb identifier.
        bomb_id: BombId,
        /// Bomb location.
        position: Position,
        /// Bomb power.
        power: u8,
    },
    /// A game tick finished executing.
    TickCompleted {
        /// Tick number.
        tick: u64,
    },
}
