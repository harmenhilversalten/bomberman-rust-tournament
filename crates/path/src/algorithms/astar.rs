use std::collections::{BinaryHeap, HashMap};

use super::Pathfinder;
use crate::{Grid, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Point,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A* pathfinding algorithm.
#[derive(Default)]
pub struct AStar;

impl AStar {
    /// Creates a new A* instance.
    pub fn new() -> Self {
        Self
    }
}

fn heuristic<G: Grid>(grid: &G, a: Point, b: Point) -> u32 {
    let manhattan = (a.x - b.x).abs() + (a.y - b.y).abs();
    let influence = grid.influence(b).max(0);
    (manhattan + influence) as u32
}

impl Pathfinder for AStar {
    fn find_path<G: Grid>(&mut self, grid: &G, start: Point, goal: Point) -> Option<Vec<Point>> {
        let mut open = BinaryHeap::new();
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut g_score: HashMap<Point, u32> = HashMap::new();

        g_score.insert(start, 0);
        open.push(Node {
            position: start,
            cost: heuristic(grid, start, goal),
        });

        while let Some(Node { position, .. }) = open.pop() {
            if position == goal {
                let mut path = vec![position];
                let mut current = position;
                while let Some(prev) = came_from.get(&current) {
                    path.push(*prev);
                    current = *prev;
                }
                path.reverse();
                return Some(path);
            }

            let current_g = g_score[&position];
            for neighbor in grid.neighbors(position) {
                let tentative = current_g + 1 + grid.influence(neighbor).max(0) as u32;
                if tentative < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, position);
                    g_score.insert(neighbor, tentative);
                    let f = tentative + heuristic(grid, neighbor, goal);
                    open.push(Node {
                        position: neighbor,
                        cost: f,
                    });
                }
            }
        }
        None
    }
}
