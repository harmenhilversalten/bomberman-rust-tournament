//! Pathfinding algorithm implementations.

use crate::{Grid, Point};

/// Trait implemented by all pathfinding algorithms.
pub trait Pathfinder {
    /// Finds a path from `start` to `goal` on the given `grid`.
    fn find_path<G: Grid>(&mut self, grid: &G, start: Point, goal: Point) -> Option<Vec<Point>>;
}

mod astar;
mod dstar_lite;
mod jps;

pub use astar::AStar;
pub use dstar_lite::DStarLite;
pub use jps::JumpPointSearch;
