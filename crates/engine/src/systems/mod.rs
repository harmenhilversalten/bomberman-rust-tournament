//! Engine systems and common System trait.

use std::sync::{Arc, RwLock};

use state::grid::{GameGrid, GridDelta};

/// Trait implemented by all engine systems.
pub trait System: Send {
    /// Name of the system.
    fn name(&self) -> &str;
    /// Run the system returning an optional grid delta to apply.
    fn run(&mut self, grid: &Arc<RwLock<GameGrid>>) -> Option<GridDelta>;
    /// Names of systems that must run before this one.
    fn dependencies(&self) -> &[&'static str] {
        &[]
    }
    /// Whether the system may run in parallel with other systems.
    fn parallelizable(&self) -> bool {
        true
    }
}

pub mod bomb_system;
pub mod explosion;
pub mod movement;
pub mod player;
pub mod powerup;

pub use bomb_system::BombSystem;
pub use explosion::ExplosionSystem;
pub use movement::MovementSystem;
pub use player::PlayerSystem;
pub use powerup::PowerupSystem;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Engine;
    use state::grid::Tile;

    #[test]
    fn systems_interact_on_grid() {
        let (mut engine, _rx) = Engine::new(2);
        engine.add_system(Box::new(MovementSystem::new()));
        engine.add_system(Box::new(PlayerSystem::new()));
        engine.add_system(Box::new(BombSystem::new()));
        engine.add_system(Box::new(ExplosionSystem::new()));
        engine.add_system(Box::new(PowerupSystem::new()));

        engine.tick();

        let grid_arc = engine.grid();
        let grid = grid_arc.read().unwrap();
        assert_eq!(grid.tile(0, 0), Some(Tile::Empty));
        assert_eq!(grid.tile(1, 0), Some(Tile::PowerUp));
    }
}
