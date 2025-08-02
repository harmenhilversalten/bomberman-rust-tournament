//! Heuristic functions for pathfinding.

use crate::Point;

/// Trait implemented by heuristic strategies.
pub trait Heuristic {
    /// Estimates the distance between two points.
    fn distance(&self, a: Point, b: Point) -> u32;
}

pub mod euclidean;
pub mod manhattan;

pub use euclidean::Euclidean;
pub use manhattan::Manhattan;
