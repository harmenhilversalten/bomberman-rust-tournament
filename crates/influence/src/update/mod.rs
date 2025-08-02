//! Update strategies for the influence map.

mod dirty_tracking;
mod full;
mod incremental;

pub use dirty_tracking::DirtyTracker;
pub use full::FullUpdate;
pub use incremental::IncrementalUpdate;

/// Strategy for determining which regions require recomputation.
pub trait UpdateStrategy: Send {
    /// Populate the provided [`DirtyTracker`] with regions that should be recomputed.
    fn update(&mut self, tracker: &mut DirtyTracker, width: u16, height: u16);
}
