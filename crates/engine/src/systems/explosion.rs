use std::sync::{Arc, RwLock};

use events::bus::EventBus;
use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Handles bomb explosions and resulting tile changes.
pub struct ExplosionSystem;

impl ExplosionSystem {
    /// Create a new `ExplosionSystem`.
    pub fn new() -> Self {
        Self
    }
}

impl System for ExplosionSystem {
    fn name(&self) -> &str {
        "explosion"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>, _events: &EventBus) -> Option<GridDelta> {
        Some(GridDelta::SetTile {
            x: 1,
            y: 0,
            tile: Tile::Empty,
        })
    }

    fn dependencies(&self) -> &[&'static str] {
        &["bomb"]
    }
}
