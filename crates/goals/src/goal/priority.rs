//! Utilities for goal priority calculations.

/// Computes a weighted priority score.
pub fn weighted_priority(priority: f32, weight: f32) -> f32 {
    priority * weight
}
