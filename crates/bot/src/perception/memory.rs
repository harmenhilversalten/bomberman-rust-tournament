//! Simple memory keeping track of recent observations.

use super::Observation;

/// Stores a history of observations for a bot.
#[derive(Debug, Default)]
pub struct BotMemory {
    observations: Vec<Observation>,
}

impl BotMemory {
    /// Create a new, empty [`BotMemory`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Remember a new [`Observation`].
    pub fn remember(&mut self, obs: Observation) {
        self.observations.push(obs);
    }

    /// Retrieve the most recent observation, if any.
    pub fn last(&self) -> Option<Observation> {
        self.observations.last().copied()
    }

    /// Number of stored observations.
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns true if no observations have been recorded.
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembering_observations_updates_memory() {
        let mut mem = BotMemory::new();
        assert_eq!(mem.len(), 0);
        mem.remember(Observation::from_snapshot(1));
        assert_eq!(mem.len(), 1);
        assert_eq!(mem.last().unwrap().value, 1);
    }
}
