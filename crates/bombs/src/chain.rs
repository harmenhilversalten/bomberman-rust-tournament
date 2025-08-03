//! Chain reaction handling for explosions.

use state::grid::GameGrid;

use crate::bomb::entity::Position;
use crate::explosion::Explosion;
use serde::{Deserialize, Serialize};

/// Result of processing a chain reaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainReaction {
    /// Positions affected by the chain reaction.
    pub positions: Vec<Position>,
}

/// Handles chain reactions between bombs.
#[derive(Default)]
pub struct ChainReactionHandler;

impl ChainReactionHandler {
    /// Create a new handler.
    pub fn new() -> Self {
        Self
    }

    /// Process potential chain reactions given explosions and mutate the grid.
    pub fn process_chain_reactions(
        &self,
        _explosions: Vec<Explosion>,
        _grid: &mut GameGrid,
    ) -> Vec<ChainReaction> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_returns_empty() {
        let handler = ChainReactionHandler::new();
        let mut grid = GameGrid::new(1, 1);
        let reactions = handler.process_chain_reactions(Vec::new(), &mut grid);
        assert!(reactions.is_empty());
    }
}
