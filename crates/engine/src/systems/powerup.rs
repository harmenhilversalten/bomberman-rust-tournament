use std::sync::{Arc, RwLock};

use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Spawns powerups after explosions clear tiles.
pub struct PowerupSystem;

impl PowerupSystem {
    /// Create a new `PowerupSystem`.
    pub fn new() -> Self {
        Self
    }
}

impl System for PowerupSystem {
    fn name(&self) -> &str {
        "powerup"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>) -> Option<GridDelta> {
        Some(GridDelta::SetTile {
            x: 1,
            y: 0,
            tile: Tile::PowerUp,
        })
    }

    fn dependencies(&self) -> &[&'static str] {
        &["explosion"]
    }
}
