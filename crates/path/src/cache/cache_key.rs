use crate::Point;

/// Unique key identifying a cached path.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CacheKey {
    /// Starting point of the path.
    pub start: Point,
    /// Goal point of the path.
    pub goal: Point,
}

impl CacheKey {
    /// Creates a new `CacheKey` from `start` and `goal` positions.
    pub fn new(start: Point, goal: Point) -> Self {
        Self { start, goal }
    }
}
