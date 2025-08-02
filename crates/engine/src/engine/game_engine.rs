use std::sync::{Arc, RwLock};

use state::{GameGrid, Tile, grid::GridDelta};
use tokio::sync::watch;

/// Core game engine advancing the simulation and broadcasting changes.
pub struct Engine {
    grid: Arc<RwLock<GameGrid>>,
    delta_tx: watch::Sender<GridDelta>,
    toggle: bool,
}

impl Engine {
    /// Creates a new engine with a square grid of the given size.
    pub fn new(size: usize) -> (Self, watch::Receiver<GridDelta>) {
        let grid = GameGrid::new(size, size);
        let (tx, rx) = watch::channel(GridDelta::None);
        (
            Self {
                grid: Arc::new(RwLock::new(grid)),
                delta_tx: tx,
                toggle: false,
            },
            rx,
        )
    }

    /// Advances the game by a single tick.
    ///
    /// This demo implementation simply toggles the tile at (0,0) between
    /// `Tile::Empty` and `Tile::Wall` and broadcasts the resulting delta.
    pub fn tick(&mut self) {
        let delta = {
            let mut grid = self.grid.write().expect("grid lock poisoned");
            let tile = if self.toggle { Tile::Empty } else { Tile::Wall };
            self.toggle = !self.toggle;
            let delta = GridDelta::SetTile { x: 0, y: 0, tile };
            grid.apply_delta(delta.clone());
            delta
        };
        let _ = self.delta_tx.send(delta);
    }

    /// Access the shared game grid.
    pub fn grid(&self) -> Arc<RwLock<GameGrid>> {
        Arc::clone(&self.grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_broadcasts_delta() {
        let (mut engine, mut rx) = Engine::new(1);
        assert_eq!(*rx.borrow(), GridDelta::None);
        engine.tick();
        assert_eq!(
            rx.borrow_and_update().clone(),
            GridDelta::SetTile {
                x: 0,
                y: 0,
                tile: Tile::Wall
            }
        );
    }
}
