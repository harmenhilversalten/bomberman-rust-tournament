//! Bomb management module aggregating bomb logic.

use std::collections::{HashMap, HashSet};

pub mod chain;
pub mod entity;
pub mod explosion;

use chain::{BombChain, find_bomb_chains};
use entity::{Bomb, BombId};
use explosion::Explosion;

use thiserror::Error;

/// Errors that can occur in bomb management.
#[derive(Debug, Error)]
pub enum BombError {
    /// Requested bomb was not found.
    #[error("bomb {0:?} not found")]
    MissingBomb(BombId),
}

/// Manages active bombs and their interactions.
#[derive(Default)]
pub struct BombManager {
    bombs: HashMap<BombId, Bomb>,
}

impl BombManager {
    /// Creates a new empty manager.
    pub fn new() -> Self {
        Self {
            bombs: HashMap::new(),
        }
    }

    /// Inserts a bomb into the manager.
    pub fn add_bomb(&mut self, bomb: Bomb) {
        self.bombs.insert(bomb.id, bomb);
    }

    /// Returns the current bomb chains based on spatial relationships.
    pub fn calculate_chain_reactions(&self) -> Vec<BombChain> {
        find_bomb_chains(&self.bombs)
    }

    /// Calculates the explosion for a given bomb on a grid with `walls`.
    pub fn calculate_explosion(
        &self,
        id: BombId,
        size: (u16, u16),
        walls: &HashSet<entity::Position>,
    ) -> Result<Explosion, BombError> {
        let bomb = self.bombs.get(&id).ok_or(BombError::MissingBomb(id))?;
        Ok(Explosion::from_bomb(bomb, size, walls))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_computes_chain_and_explosion() {
        let mut mgr = BombManager::new();
        let b1 = Bomb::new(BombId(1), 0, (1, 1), 1, 2);
        let b2 = Bomb::new(BombId(2), 0, (3, 1), 5, 2);
        mgr.add_bomb(b1.clone());
        mgr.add_bomb(b2.clone());

        let chains = mgr.calculate_chain_reactions();
        assert_eq!(chains.len(), 1);

        let explosion = mgr
            .calculate_explosion(b1.id, (5, 5), &HashSet::new())
            .unwrap();
        assert!(explosion.affected_cells.contains(&b2.position));
    }
}
