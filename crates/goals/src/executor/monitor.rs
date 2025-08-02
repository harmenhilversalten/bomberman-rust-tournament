//! Progress monitoring utilities for goal execution.

/// Tracks progress over ticks and detects stalls.
#[derive(Debug, Clone)]
pub struct ProgressMonitor {
    last_progress: f32,
    stagnant_ticks: u32,
    max_stagnant_ticks: u32,
}

impl ProgressMonitor {
    /// Creates a new monitor that flags a stall after `max_stagnant_ticks`.
    pub fn new(max_stagnant_ticks: u32) -> Self {
        Self {
            last_progress: 0.0,
            stagnant_ticks: 0,
            max_stagnant_ticks,
        }
    }

    /// Updates the monitor with the latest progress measurement.
    pub fn update(&mut self, progress: f32) {
        if progress > self.last_progress {
            self.last_progress = progress;
            self.stagnant_ticks = 0;
        } else {
            self.stagnant_ticks += 1;
        }
    }

    /// Returns true if the monitored goal appears stalled.
    pub fn is_stalled(&self) -> bool {
        self.stagnant_ticks >= self.max_stagnant_ticks
    }
}
