use std::time::Duration;

/// Runtime state tracked by a [`Bot`].
#[derive(Default, Debug)]
pub struct BotState {
    decisions: usize,
    last_duration: Option<Duration>,
}

impl BotState {
    /// Record that a decision was made taking `duration`.
    pub fn record_decision(&mut self, duration: Duration) {
        self.decisions += 1;
        self.last_duration = Some(duration);
    }

    /// Number of decisions made.
    pub fn decisions(&self) -> usize {
        self.decisions
    }

    /// Duration of the last decision, if any.
    pub fn last_duration(&self) -> Option<Duration> {
        self.last_duration
    }
}
