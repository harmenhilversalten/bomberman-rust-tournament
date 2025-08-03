//! High level bomb logic wrappers.

use state::grid::{GameGrid, GridDelta};

/// Handles bomb updates each tick.
#[derive(Default)]
pub struct BombLogic;

impl BombLogic {
    /// Create new bomb logic instance.
    pub fn new() -> Self {
        Self
    }

    /// Update bombs on the grid returning any state delta.
    pub fn update_bombs(&mut self, _grid: &mut GameGrid) -> GridDelta {
        GridDelta::None
    }
}

/// Placeholder state type for bombs.
pub struct BombState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_returns_none_delta() {
        let mut logic = BombLogic::new();
        let mut grid = GameGrid::new(1, 1);
        assert!(matches!(logic.update_bombs(&mut grid), GridDelta::None));
    }
}
