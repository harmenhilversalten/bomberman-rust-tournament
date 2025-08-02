//! Game state wrapper used by other crates.

use crate::grid::GameGrid;

/// Main game state structure.
#[derive(Debug)]
pub struct GameState {
    /// Underlying grid data
    pub grid: GameGrid,
}

impl GameState {
    /// Creates a new game state with an empty grid.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: GameGrid::new(width, height),
        }
    }

    /// Apply a delta to the underlying grid.
    pub fn apply_delta(&mut self, delta: crate::grid::GridDelta) {
        self.grid.apply_delta(delta);
    }

    /// Subscribe to grid updates.
    pub fn subscribe(&self) -> tokio::sync::watch::Receiver<crate::grid::GridDelta> {
        self.grid.subscribe()
    }

    /// Convert state to a flat observation for the specified agent.
    pub fn to_observation(&self, agent_id: usize) -> Vec<f32> {
        self.grid.to_observation(agent_id)
    }
}
