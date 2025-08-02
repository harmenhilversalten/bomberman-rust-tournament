//! Incremental update strategy which only recomputes already marked regions.

use super::{DirtyTracker, UpdateStrategy};

/// Strategy that performs no additional work beyond existing dirty regions.
#[derive(Default)]
pub struct IncrementalUpdate;

impl IncrementalUpdate {
    /// Creates a new [`IncrementalUpdate`] instance.
    pub fn new() -> Self {
        Self
    }
}

impl UpdateStrategy for IncrementalUpdate {
    fn update(&mut self, _tracker: &mut DirtyTracker, _width: u16, _height: u16) {
        // Nothing to do; dirty regions are supplied externally.
    }
}
