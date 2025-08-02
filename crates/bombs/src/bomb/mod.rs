//! Bomb management module aggregating bomb logic.

use std::collections::{HashMap, HashSet};

pub mod chain;
pub mod entity;
pub mod explosion;

use crate::timing::{BombTimer, RemoteDetonator};
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
    /// The bomb is not configured for remote detonation.
    #[error("bomb {0:?} is not remote-capable")]
    NotRemote(BombId),
}

/// Manages active bombs and their interactions.
#[derive(Default)]
pub struct BombManager {
    bombs: HashMap<BombId, Bomb>,
    timers: HashMap<BombId, BombTimer>,
    remote: RemoteDetonator,
}

impl BombManager {
    /// Creates a new empty manager.
    pub fn new() -> Self {
        Self {
            bombs: HashMap::new(),
            timers: HashMap::new(),
            remote: RemoteDetonator::default(),
        }
    }

    /// Inserts a bomb into the manager.
    pub fn add_bomb(&mut self, bomb: Bomb) {
        if bomb.remote {
            self.remote.arm(bomb.id);
        }
        self.timers.insert(bomb.id, BombTimer::new(bomb.timer));
        self.bombs.insert(bomb.id, bomb);
    }

    /// Advances all bomb timers by one tick and returns bombs ready to explode.
    pub fn tick(&mut self) -> Vec<BombId> {
        let mut ready = Vec::new();
        for (&id, timer) in self.timers.iter_mut() {
            if timer.tick() {
                ready.push(id);
            }
        }
        ready
    }

    /// Detonates a remote-capable bomb immediately.
    pub fn detonate_remote(&mut self, id: BombId) -> Result<(), BombError> {
        if self.remote.detonate(id) {
            if let Some(timer) = self.timers.get_mut(&id) {
                *timer = BombTimer::new(0);
            }
            Ok(())
        } else {
            Err(BombError::NotRemote(id))
        }
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

    #[test]
    fn ticking_triggers_bomb() {
        let mut mgr = BombManager::new();
        let bomb = Bomb::new(BombId(3), 0, (0, 0), 1, 1);
        mgr.add_bomb(bomb);
        let ready = mgr.tick();
        assert_eq!(ready, vec![BombId(3)]);
    }

    #[test]
    fn remote_detonation_marks_bomb_ready() {
        let mut mgr = BombManager::new();
        let mut bomb = Bomb::new(BombId(4), 0, (0, 0), 5, 1);
        bomb.remote = true;
        mgr.add_bomb(bomb);
        assert!(mgr.detonate_remote(BombId(4)).is_ok());
        let ready = mgr.tick();
        assert_eq!(ready, vec![BombId(4)]);
    }
}
