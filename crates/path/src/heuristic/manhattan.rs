//! Manhattan distance heuristic.

use crate::Point;

use super::Heuristic;

/// Manhattan distance estimator.
#[derive(Default)]
pub struct Manhattan;

impl Heuristic for Manhattan {
    fn distance(&self, a: Point, b: Point) -> u32 {
        ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
    }
}
