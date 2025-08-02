//! Placement strategy trait.

use crate::bomb::entity::Position;

/// Strategy for choosing a bomb placement from available options.
pub trait PlacementStrategy {
    /// Selects a position from `options` according to the strategy.
    fn choose(&self, options: &[Position]) -> Option<Position>;
}
