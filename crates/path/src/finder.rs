//! High-level pathfinder wrapper.

use crate::Point;
use crate::path::{Path, PathNode};
use influence::map::InfluenceData;

/// High level pathfinder selecting algorithms.
#[derive(Default)]
pub struct Pathfinder;

/// Marker trait for pathfinding algorithms.
pub trait PathfindingAlgorithm {}

impl Pathfinder {
    /// Create a new [`Pathfinder`].
    pub fn new() -> Self {
        Self
    }

    /// Find a path between two points considering influence data.
    pub fn find_path(&self, start: Point, goal: Point, _influence: &InfluenceData) -> Option<Path> {
        // Placeholder: straight line path with start and goal nodes.
        let nodes = vec![PathNode { position: start }, PathNode { position: goal }];
        Some(Path::new(nodes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use influence::map::InfluenceMap;

    #[test]
    fn pathfinder_returns_simple_path() {
        let finder = Pathfinder::new();
        let map = InfluenceMap::new(1, 1);
        let path = finder
            .find_path(Point::new(0, 0), Point::new(2, 0), &map.data())
            .unwrap();
        assert_eq!(path.nodes.len(), 2);
    }
}
