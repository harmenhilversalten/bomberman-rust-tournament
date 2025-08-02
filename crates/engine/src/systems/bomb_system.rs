use std::sync::{Arc, RwLock};

use bombs::bomb::entity::{Bomb, BombId};
use events::{
    bus::EventBus,
    events::{Event, GameEvent},
};
use state::grid::{GameGrid, GridDelta, Tile};

use super::System;

/// Manages bomb placement and updates.
pub struct BombSystem {
    next_id: u32,
}

impl BombSystem {
    /// Create a new `BombSystem`.
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
}

impl System for BombSystem {
    fn name(&self) -> &str {
        "bomb"
    }

    fn run(&mut self, _grid: &Arc<RwLock<GameGrid>>, events: &EventBus) -> Option<GridDelta> {
        let bomb = Bomb::new(BombId(self.next_id), 0, (0, 0), 3, 1);
        self.next_id += 1;
        events.broadcast(Event::Game(GameEvent::BombPlaced {
            entity_id: bomb.owner,
            bomb_id: bomb.id.0 as usize,
            position: bomb.position,
            power: bomb.power,
        }));
        Some(GridDelta::SetTile {
            x: 1,
            y: 0,
            tile: Tile::SoftCrate,
        })
    }
}
