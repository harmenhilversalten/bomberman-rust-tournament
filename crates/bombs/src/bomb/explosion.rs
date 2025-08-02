//! Explosion calculation and blast radius.

use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::graphmap::UnGraphMap;

use super::entity::{Bomb, BombId, Position};

/// Result of a bomb explosion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Explosion {
    /// Bomb responsible for the explosion.
    pub bomb_id: BombId,
    /// All cells affected by the blast including the bomb's own position.
    pub affected_cells: Vec<Position>,
}

impl Explosion {
    /// Calculate explosion for a bomb on a grid of `size` with immutable `walls`.
    pub fn from_bomb(bomb: &Bomb, size: (u16, u16), walls: &HashSet<Position>) -> Self {
        let affected = blast_radius(bomb, size, walls);
        Self {
            bomb_id: bomb.id,
            affected_cells: affected,
        }
    }
}

/// Calculate positions reached by a bomb's explosion using BFS.
fn blast_radius(bomb: &Bomb, size: (u16, u16), walls: &HashSet<Position>) -> Vec<Position> {
    let mut graph = UnGraphMap::<Position, ()>::new();
    let (width, height) = size;
    for x in 0..width {
        for y in 0..height {
            let pos = (x, y);
            if walls.contains(&pos) {
                continue;
            }
            graph.add_node(pos);
        }
    }
    for x in 0..width {
        for y in 0..height {
            let pos = (x, y);
            if walls.contains(&pos) {
                continue;
            }
            if x + 1 < width && !walls.contains(&(x + 1, y)) {
                graph.add_edge(pos, (x + 1, y), ());
            }
            if x > 0 && !walls.contains(&(x - 1, y)) {
                graph.add_edge(pos, (x - 1, y), ());
            }
            if y + 1 < height && !walls.contains(&(x, y + 1)) {
                graph.add_edge(pos, (x, y + 1), ());
            }
            if y > 0 && !walls.contains(&(x, y - 1)) {
                graph.add_edge(pos, (x, y - 1), ());
            }
        }
    }

    let mut dist: HashMap<Position, u8> = HashMap::new();
    let mut q = VecDeque::new();
    dist.insert(bomb.position, 0);
    q.push_back(bomb.position);

    while let Some(pos) = q.pop_front() {
        let d = dist[&pos];
        if d >= bomb.power {
            continue;
        }
        for neigh in graph.neighbors(pos) {
            if let std::collections::hash_map::Entry::Vacant(e) = dist.entry(neigh) {
                e.insert(d + 1);
                q.push_back(neigh);
            }
        }
    }

    let mut cells: Vec<_> = dist
        .into_iter()
        .filter(|(_, d)| *d <= bomb.power)
        .map(|(p, _)| p)
        .collect();
    cells.sort();
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blast_respects_walls() {
        let bomb = Bomb::new(BombId(1), 0, (1, 1), 0, 3);
        let mut walls = HashSet::new();
        walls.insert((2, 1));
        let explosion = Explosion::from_bomb(&bomb, (5, 5), &walls);
        assert!(explosion.affected_cells.contains(&(0, 1))); // left
        assert!(!explosion.affected_cells.contains(&(3, 1))); // blocked by wall
        assert!(explosion.affected_cells.contains(&(1, 4))); // down
    }
}
