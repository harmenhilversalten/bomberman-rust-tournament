//! Tile representation for the game grid.

use serde::{Deserialize, Serialize};

/// Different types of grid tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    /// Empty walkable tile
    Empty,
    /// Indestructible wall
    Wall,
    /// Destructible crate
    SoftCrate,
    /// Tile containing a power-up
    PowerUp,
    /// Explosion animation tile (temporary)
    Explosion,
}

impl Tile {
    /// Serialize tile to a numeric representation.
    pub fn to_u8(self) -> u8 {
        match self {
            Tile::Empty => 0,
            Tile::Wall => 1,
            Tile::SoftCrate => 2,
            Tile::PowerUp => 3,
            Tile::Explosion => 4,
        }
    }
}
