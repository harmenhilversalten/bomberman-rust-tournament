//! Tile representation for the game grid.

/// Different types of grid tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// Empty walkable tile
    Empty,
    /// Indestructible wall
    Wall,
    /// Destructible crate
    SoftCrate,
    /// Tile containing a power-up
    PowerUp,
}
