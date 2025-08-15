use super::tile::Tile;
use crate::components::{AgentState, Bomb};
use serde::{Deserialize, Serialize};

/// Changes applied to the grid, broadcast to subscribers.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum GridDelta {
    /// No change, used as initial value for watchers.
    #[default]
    None,
    /// Set a tile at a specific position.
    SetTile {
        /// X coordinate of the tile
        x: usize,
        /// Y coordinate of the tile
        y: usize,
        /// New tile value
        tile: Tile,
    },
    /// Add a bomb to the grid.
    AddBomb(Bomb),
    /// Add an agent to the grid.
    AddAgent(AgentState),
    /// Move an agent to a new position.
    MoveAgent(usize, (u16, u16)),
    /// Remove an agent from the grid.
    RemoveAgent(usize),
}
