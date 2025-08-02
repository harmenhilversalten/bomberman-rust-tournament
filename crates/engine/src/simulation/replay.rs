use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use state::{GameGrid, grid::GridDelta};

/// Recorded sequence of [`GridDelta`] events.
#[derive(Clone, Debug, Default)]
pub struct Replay {
    deltas: Vec<GridDelta>,
}

impl Replay {
    /// Create a replay from raw deltas.
    pub fn new(deltas: Vec<GridDelta>) -> Self {
        Self { deltas }
    }

    /// Access recorded deltas.
    pub fn deltas(&self) -> &[GridDelta] {
        &self.deltas
    }

    /// Apply the replay to a [`GameGrid`].
    pub fn apply(&self, grid: &mut GameGrid) {
        for delta in &self.deltas {
            grid.apply_delta(delta.clone());
        }
    }
}

/// Utility for recording grid deltas during simulation.
#[derive(Clone, Default)]
pub struct ReplayRecorder {
    recording: Arc<AtomicBool>,
    deltas: Arc<Mutex<Vec<GridDelta>>>,
}

impl ReplayRecorder {
    /// Create a new recorder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start recording deltas.
    pub fn start(&self) {
        self.deltas.lock().expect("recorder lock poisoned").clear();
        self.recording.store(true, Ordering::SeqCst);
    }

    /// Record a delta if recording is active.
    pub fn record(&self, delta: GridDelta) {
        if self.recording.load(Ordering::SeqCst) {
            self.deltas
                .lock()
                .expect("recorder lock poisoned")
                .push(delta);
        }
    }

    /// Stop recording and return the collected replay.
    pub fn stop(&self) -> Replay {
        self.recording.store(false, Ordering::SeqCst);
        Replay {
            deltas: self.deltas.lock().expect("recorder lock poisoned").clone(),
        }
    }
}
