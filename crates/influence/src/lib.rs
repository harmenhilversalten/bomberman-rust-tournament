//! Influence map crate providing danger and opportunity layers.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Core influence map functionality.
pub mod core;
/// Layer type aliases.
pub mod layers;
/// Simplified map wrappers.
pub mod map;
/// Update strategies and dirty region tracking.
pub mod update;
/// Visualization and export helpers.
pub mod visualization;

pub use core::{DangerSource, DirtyRegion, InfluenceError, InfluenceType, OpportunitySource};
pub use layers::{DangerLayer, OpportunityLayer};
pub use map::{InfluenceData, InfluenceMap};
pub use update::{FullUpdate, IncrementalUpdate, UpdateStrategy};
