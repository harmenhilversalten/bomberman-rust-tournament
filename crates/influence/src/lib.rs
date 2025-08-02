//! Influence map crate providing danger and opportunity layers.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Core influence map functionality.
pub mod core;
/// Update strategies and dirty region tracking.
pub mod update;
/// Visualization and export helpers.
pub mod visualization;

pub use core::{
    DangerSource, DirtyRegion, InfluenceError, InfluenceMap, InfluenceType, OpportunitySource,
};
pub use update::{FullUpdate, IncrementalUpdate, UpdateStrategy};
