use std::sync::{Arc, RwLock};

use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Updates player related state.
pub struct PlayerSystem;

impl PlayerSystem {
    /// Create a new `PlayerSystem`.
    pub fn new() -> Self {
        Self
    }
}

impl System for PlayerSystem {
    fn name(&self) -> &str {
        "player"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>) -> Option<GridDelta> {
        Some(GridDelta::SetTile {
            x: 0,
            y: 0,
            tile: Tile::Empty,
        })
    }

    fn dependencies(&self) -> &[&'static str] {
        &["movement"]
    }
}
