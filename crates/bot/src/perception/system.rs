//! Perception system updating observations and memory.

use super::{BotMemory, Observation};

/// Converts raw snapshots into observations while maintaining memory.
#[derive(Debug, Default)]
pub struct PerceptionSystem {
    memory: BotMemory,
}

impl PerceptionSystem {
    /// Create a new [`PerceptionSystem`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a snapshot, storing it in memory and returning the [`Observation`].
    pub fn update(&mut self, snapshot: i32) -> Observation {
        let obs = Observation::from_snapshot(snapshot);
        self.memory.remember(obs);
        obs
    }

    /// Access the underlying memory.
    pub fn memory(&self) -> &BotMemory {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_stores_observation_in_memory() {
        let mut sys = PerceptionSystem::new();
        let obs = sys.update(5);
        assert_eq!(obs.value, 5);
        assert_eq!(sys.memory().last().unwrap().value, 5);
    }
}
