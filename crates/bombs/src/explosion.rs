//! Explosion calculation utilities.

use serde::{Deserialize, Serialize};
use state::grid::GameGrid;

use crate::bomb::entity::Position;

/// Description of a bomb explosion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explosion {
    /// Center position of the explosion.
    pub position: Position,
    /// Blast radius.
    pub radius: u32,
}

/// Collection of tiles affected by an explosion.
pub type BlastPattern = Vec<Position>;

/// Calculates explosions for bombs on the grid.
#[derive(Default)]
pub struct ExplosionCalculator;

impl ExplosionCalculator {
    /// Create a new calculator.
    pub fn new() -> Self {
        Self
    }

    /// Determine explosions for bombs currently on the grid.
    pub fn calculate_explosions(&self, _grid: &GameGrid) -> Vec<Explosion> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator_returns_empty() {
        let calc = ExplosionCalculator::new();
        let grid = GameGrid::new(1, 1);
        assert!(calc.calculate_explosions(&grid).is_empty());
    }
}
