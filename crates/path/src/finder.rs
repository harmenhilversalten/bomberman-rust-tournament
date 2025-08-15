//! High-level pathfinder wrapper.

use crate::Point;
use crate::path::{Path, PathNode};
use crate::algorithms::{AStar, Pathfinder as PathfinderTrait};
use crate::Grid;
use influence::map::InfluenceData;

/// High level pathfinder selecting algorithms.
pub struct Pathfinder {
    algorithm: AStar,
}

/// Marker trait for pathfinding algorithms.
pub trait PathfindingAlgorithm {}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Pathfinder {
    /// Create a new [`Pathfinder`].
    pub fn new() -> Self {
        Self {
            algorithm: AStar::new(),
        }
    }

    /// Find a path between two points considering influence data.
    pub fn find_path(&mut self, start: Point, goal: Point, influence: &InfluenceData) -> Option<Path> {
        // Create a grid adapter that incorporates influence data
        let grid = InfluenceGrid::new(influence);
        
        // Use A* to find the path
        if let Some(points) = self.algorithm.find_path(&grid, start, goal) {
            let nodes = points.into_iter().map(|p| PathNode { position: p }).collect();
            Some(Path::new(nodes))
        } else {
            None
        }
    }
}

/// Grid adapter that incorporates influence data
struct InfluenceGrid<'a> {
    influence: &'a InfluenceData<'a>,
}

impl<'a> InfluenceGrid<'a> {
    fn new(influence: &'a InfluenceData<'a>) -> Self {
        Self { influence }
    }
}

impl<'a> Grid for InfluenceGrid<'a> {
    fn width(&self) -> i32 {
        self.influence.width() as i32
    }

    fn height(&self) -> i32 {
        self.influence.height() as i32
    }

    fn is_walkable(&self, p: Point) -> bool {
        if p.x < 0 || p.y < 0 || p.x >= self.width() || p.y >= self.height() {
            return false;
        }
        
        // Use the influence API to get danger at this position
        let position = influence::map::Position::new(p.x, p.y);
        let danger = self.influence.get_danger_at(position);
        
        // Consider positions with high danger as unwalkable (walls/obstacles)
        danger < 100.0
    }

    fn influence(&self, p: Point) -> i32 {
        if p.x < 0 || p.y < 0 || p.x >= self.width() || p.y >= self.height() {
            return 1000; // High penalty for out-of-bounds
        }
        
        let position = influence::map::Position::new(p.x, p.y);
        let danger = self.influence.get_danger_at(position);
        danger as i32
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
            .find_path(Point::new(0, 0), Point::new(0, 0), &map.data())
            .unwrap();
        assert_eq!(path.nodes.len(), 2);
    }
}
