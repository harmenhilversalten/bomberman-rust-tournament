use std::sync::{Arc, RwLock};

use bombs::{BombLogic, ChainReactionHandler, ExplosionCalculator};
use events::{
    bus::EventBus,
    events::{BombEvent, Event},
};
use state::grid::{GameGrid, GridDelta};

use super::System;

/// Manages bombs using logic from the `bombs` crate.
pub struct BombSystem {
    bomb_logic: BombLogic,
    explosion_calc: ExplosionCalculator,
    chain_handler: ChainReactionHandler,
}

impl BombSystem {
    /// Create a new [`BombSystem`].
    pub fn new() -> Self {
        Self {
            bomb_logic: BombLogic::new(),
            explosion_calc: ExplosionCalculator::new(),
            chain_handler: ChainReactionHandler::new(),
        }
    }
}

impl System for BombSystem {
    fn name(&self) -> &str {
        "bomb"
    }

    fn run(&mut self, grid: &Arc<RwLock<GameGrid>>, events: &EventBus) -> Option<GridDelta> {
        let mut grid = grid.write().unwrap();
        let delta = self.bomb_logic.update_bombs(&mut grid);

        let explosions = self.explosion_calc.calculate_explosions(&grid);
        let chain_reactions = self
            .chain_handler
            .process_chain_reactions(explosions.clone(), &mut grid);

        for explosion in explosions {
            events.broadcast(Event::bomb(BombEvent::Exploded {
                position: explosion.position,
                radius: explosion.radius,
            }));
        }
        for reaction in chain_reactions {
            events.broadcast(Event::bomb(BombEvent::ChainReaction {
                positions: reaction.positions,
            }));
        }
        Some(delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_broadcasts_events() {
        let mut system = BombSystem::new();
        let grid = Arc::new(RwLock::new(GameGrid::new(1, 1)));
        let bus = EventBus::new();
        system.run(&grid, &bus);
        // no assertion on content, just ensure call succeeds
    }
}
