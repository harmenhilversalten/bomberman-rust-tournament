//! Grid implementation used for pathfinding.

use super::Node;
use crate::{Grid, Point};

/// Grid backing pathfinding algorithms.
#[derive(Clone)]
pub struct PathGrid {
    width: i32,
    height: i32,
    nodes: Vec<Node>,
}

impl PathGrid {
    /// Creates a new `PathGrid` with the given dimensions.
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            nodes: vec![Node::default(); size],
        }
    }

    fn index(&self, p: Point) -> usize {
        (p.y * self.width + p.x) as usize
    }

    /// Marks a cell as walkable or blocked.
    pub fn set_walkable(&mut self, p: Point, walkable: bool) {
        let idx = self.index(p);
        self.nodes[idx].walkable = walkable;
    }

    /// Sets the movement cost of a cell. Minimum cost is 1.
    pub fn set_cost(&mut self, p: Point, cost: u32) {
        let idx = self.index(p);
        self.nodes[idx].cost = cost.max(1);
    }

    pub(crate) fn node_cost(&self, p: Point) -> u32 {
        let idx = self.index(p);
        self.nodes[idx].cost
    }
}

impl Grid for PathGrid {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn is_walkable(&self, p: Point) -> bool {
        let idx = self.index(p);
        self.nodes[idx].walkable
    }

    fn influence(&self, p: Point) -> i32 {
        let idx = self.index(p);
        self.nodes[idx].cost as i32 - 1
    }
}
