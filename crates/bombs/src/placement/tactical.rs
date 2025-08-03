//! Tactical bomb placement scoring.

use crate::bomb::entity::Position;
use state::grid::GameGrid;

/// Trait for evaluating bomb placements.
pub trait BombPlacementStrategy {
    /// Evaluate placing a bomb at `position` within `snapshot`.
    fn evaluate_placement(&self, position: Position, snapshot: &GameGrid) -> f32;
}

/// Basic tactical placement strategy.
pub struct TacticalPlacement;

impl BombPlacementStrategy for TacticalPlacement {
    fn evaluate_placement(&self, _position: Position, _snapshot: &GameGrid) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tactical_returns_zero_score() {
        let strat = TacticalPlacement;
        let grid = GameGrid::new(1, 1);
        assert_eq!(strat.evaluate_placement((0, 0), &grid), 0.0);
    }
}
