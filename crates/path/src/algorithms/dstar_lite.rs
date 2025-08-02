use super::{AStar, Pathfinder};
use crate::{Grid, Point};

/// Simplified D* Lite algorithm.
///
/// This implementation delegates to A* and acts as a placeholder for a
/// full D* Lite incremental search.
#[derive(Default)]
pub struct DStarLite {
    inner: AStar,
}

impl DStarLite {
    /// Creates a new D* Lite instance.
    pub fn new() -> Self {
        Self {
            inner: AStar::new(),
        }
    }
}

impl Pathfinder for DStarLite {
    fn find_path<G: Grid>(&mut self, grid: &G, start: Point, goal: Point) -> Option<Vec<Point>> {
        self.inner.find_path(grid, start, goal)
    }
}
