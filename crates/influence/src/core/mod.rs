//! Core module containing influence map structures and layers.

/// Danger layer implementation.
pub mod danger;
/// Influence map container and related types.
pub mod influence_map;
/// Layer trait definition.
pub mod layer;
/// Opportunity layer implementation.
pub mod opportunity;

pub use danger::{DangerMap, DangerSource};
pub use influence_map::{DirtyRegion, InfluenceError, InfluenceMap, InfluenceType};
pub use opportunity::{OpportunityMap, OpportunitySource};
