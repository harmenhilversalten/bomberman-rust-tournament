use std::sync::{Arc, RwLock};

use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Manages bomb placement and updates.
pub struct BombSystem;

impl BombSystem {
    /// Create a new `BombSystem`.
    pub fn new() -> Self {
        Self
    }
}

impl System for BombSystem {
    fn name(&self) -> &str {
        "bomb"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>) -> Option<GridDelta> {
        Some(GridDelta::SetTile {
            x: 1,
            y: 0,
            tile: Tile::SoftCrate,
        })
    }
}
