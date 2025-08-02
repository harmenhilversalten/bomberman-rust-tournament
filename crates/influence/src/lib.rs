//! Influence map crate providing danger and opportunity layers.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Core influence map functionality.
pub mod core;

pub use core::{
    DangerSource, DirtyRegion, InfluenceError, InfluenceMap, InfluenceType, OpportunitySource,
};
