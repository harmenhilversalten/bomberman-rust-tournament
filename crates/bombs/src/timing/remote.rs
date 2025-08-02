//! Remote detonation tracking for bombs.

use std::collections::HashSet;

use crate::bomb::entity::BombId;

/// Tracks bombs that can be detonated remotely.
#[derive(Default)]
pub struct RemoteDetonator {
    armed: HashSet<BombId>,
}

impl RemoteDetonator {
    /// Arms a bomb for remote detonation.
    pub fn arm(&mut self, id: BombId) {
        self.armed.insert(id);
    }

    /// Attempts to detonate the given bomb. Returns `true` if the bomb was armed.
    pub fn detonate(&mut self, id: BombId) -> bool {
        self.armed.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_and_detonate() {
        let mut remote = RemoteDetonator::default();
        let id = BombId(1);
        remote.arm(id);
        assert!(remote.detonate(id));
        assert!(!remote.detonate(id));
    }
}
