use std::sync::{Arc, RwLock};

use events::bus::EventBus;
use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Handles entity movement on the grid.
pub struct MovementSystem {
    toggle: bool,
}

impl MovementSystem {
    /// Create a new `MovementSystem`.
    pub fn new() -> Self {
        Self { toggle: false }
    }
}

impl System for MovementSystem {
    fn name(&self) -> &str {
        "movement"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>, _events: &EventBus) -> Option<GridDelta> {
        let tile = if self.toggle { Tile::Empty } else { Tile::Wall };
        self.toggle = !self.toggle;
        Some(GridDelta::SetTile { x: 0, y: 0, tile })
    }

    fn parallelizable(&self) -> bool {
        false
    }
}
