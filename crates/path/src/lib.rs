//! Pathfinding utilities for Bomberman bots.
#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

/// Basic 2D position.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: i32,
    /// Vertical coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a new `Point`.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Grid abstraction used by pathfinding algorithms.
pub trait Grid {
    /// Width of the grid.
    fn width(&self) -> i32;
    /// Height of the grid.
    fn height(&self) -> i32;
    /// Returns whether the given position is walkable.
    fn is_walkable(&self, p: Point) -> bool;
    /// Influence penalty from an influence map.
    fn influence(&self, _p: Point) -> i32 {
        0
    }
    /// Returns walkable neighbor positions of `p`.
    fn neighbors(&self, p: Point) -> Vec<Point> {
        let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut n = Vec::with_capacity(4);
        for (dx, dy) in deltas {
            let np = Point::new(p.x + dx, p.y + dy);
            if np.x >= 0
                && np.x < self.width()
                && np.y >= 0
                && np.y < self.height()
                && self.is_walkable(np)
            {
                n.push(np);
            }
        }
        n
    }
}

pub mod algorithms;

pub use algorithms::{AStar, DStarLite, JumpPointSearch, Pathfinder};
