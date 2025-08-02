use super::{AStar, Pathfinder};
use crate::{Grid, Point};

/// Jump Point Search algorithm.
///
/// Currently this is a thin wrapper around A*; a full JPS implementation
/// can be added later for performance improvements.
#[derive(Default)]
pub struct JumpPointSearch {
    inner: AStar,
}

impl JumpPointSearch {
    /// Creates a new Jump Point Search instance.
    pub fn new() -> Self {
        Self {
            inner: AStar::new(),
        }
    }
}

impl Pathfinder for JumpPointSearch {
    fn find_path<G: Grid>(&mut self, grid: &G, start: Point, goal: Point) -> Option<Vec<Point>> {
        self.inner.find_path(grid, start, goal)
    }
}
