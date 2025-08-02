//! Utilities for tracking dirty regions of the influence map.

use crate::core::DirtyRegion;

/// Tracks regions that require recomputation.
#[derive(Default)]
pub struct DirtyTracker {
    regions: Vec<DirtyRegion>,
}

impl DirtyTracker {
    /// Creates a new tracker.
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Mark a region as dirty, merging with existing overlapping regions.
    pub fn mark(&mut self, mut region: DirtyRegion) {
        let mut i = 0;
        while i < self.regions.len() {
            if overlaps(self.regions[i], region) {
                region = merge(self.regions[i], region);
                self.regions.remove(i);
            } else {
                i += 1;
            }
        }
        self.regions.push(region);
    }

    /// Returns the current list of dirty regions.
    pub fn regions(&self) -> &[DirtyRegion] {
        &self.regions
    }

    /// Clears all tracked regions.
    pub fn clear(&mut self) {
        self.regions.clear();
    }
}

fn overlaps(a: DirtyRegion, b: DirtyRegion) -> bool {
    let ax2 = u32::from(a.x) + u32::from(a.width);
    let ay2 = u32::from(a.y) + u32::from(a.height);
    let bx2 = u32::from(b.x) + u32::from(b.width);
    let by2 = u32::from(b.y) + u32::from(b.height);
    !(ax2 <= u32::from(b.x)
        || bx2 <= u32::from(a.x)
        || ay2 <= u32::from(b.y)
        || by2 <= u32::from(a.y))
}

fn merge(a: DirtyRegion, b: DirtyRegion) -> DirtyRegion {
    let x1 = a.x.min(b.x);
    let y1 = a.y.min(b.y);
    let x2 = (u32::from(a.x) + u32::from(a.width)).max(u32::from(b.x) + u32::from(b.width));
    let y2 = (u32::from(a.y) + u32::from(a.height)).max(u32::from(b.y) + u32::from(b.height));
    DirtyRegion {
        x: x1,
        y: y1,
        width: (x2 - u32::from(x1)) as u16,
        height: (y2 - u32::from(y1)) as u16,
    }
}
