//! Euclidean distance heuristic.

use crate::Point;

use super::Heuristic;

/// Euclidean distance estimator.
#[derive(Default)]
pub struct Euclidean;

impl Heuristic for Euclidean {
    fn distance(&self, a: Point, b: Point) -> u32 {
        let dx = (a.x - b.x) as f64;
        let dy = (a.y - b.y) as f64;
        ((dx * dx + dy * dy).sqrt()) as u32
    }
}
