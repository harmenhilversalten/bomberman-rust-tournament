//! Full update strategy which marks the entire map as dirty each tick.

use super::{DirtyTracker, UpdateStrategy};
use crate::core::DirtyRegion;

/// Strategy that always recomputes the full map.
#[derive(Default)]
pub struct FullUpdate;

impl FullUpdate {
    /// Creates a new [`FullUpdate`] instance.
    pub fn new() -> Self {
        Self
    }
}

impl UpdateStrategy for FullUpdate {
    fn update(&mut self, tracker: &mut DirtyTracker, width: u16, height: u16) {
        tracker.mark(DirtyRegion {
            x: 0,
            y: 0,
            width,
            height,
        });
    }
}
