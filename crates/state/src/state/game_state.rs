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
}
