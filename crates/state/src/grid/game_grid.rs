//! Core game grid storing tiles and entities.
use std::sync::atomic::{AtomicU64, Ordering};

use super::tile::Tile;
use crate::components::{AgentState, Bomb};

/// Main game grid structure holding tiles and entities.
#[derive(Debug)]
pub struct GameGrid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    bombs: Vec<Bomb>,
    agents: Vec<AgentState>,
    version: AtomicU64,
}

impl GameGrid {
    /// Creates a new grid filled with `Tile::Empty` tiles.
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![Tile::Empty; width * height];
        Self {
            width,
            height,
            tiles,
            bombs: Vec::new(),
            agents: Vec::new(),
            version: AtomicU64::new(0),
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Returns the tile at the given coordinates if within bounds.
    pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[self.index(x, y)])
        } else {
            None
        }
    }

    /// Sets the tile value and bumps the version counter.
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.tiles[idx] = tile;
            self.version.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Adds a bomb to the grid and returns its identifier.
    pub fn add_bomb(&mut self, bomb: Bomb) -> usize {
        self.bombs.push(bomb);
        self.version.fetch_add(1, Ordering::Relaxed);
        self.bombs.len() - 1
    }

    /// Adds an agent to the grid and returns its identifier.
    pub fn add_agent(&mut self, agent: AgentState) -> usize {
        self.agents.push(agent);
        self.version.fetch_add(1, Ordering::Relaxed);
        self.agents.len() - 1
    }

    /// Current version of the grid.
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_has_correct_size() {
        let grid = GameGrid::new(4, 3);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.tiles.len(), 12);
        assert_eq!(grid.version(), 0);
    }

    #[test]
    fn set_tile_updates_version() {
        let mut grid = GameGrid::new(2, 2);
        grid.set_tile(0, 1, Tile::Wall);
        assert_eq!(grid.tile(0, 1), Some(Tile::Wall));
        assert_eq!(grid.version(), 1);
    }

    #[test]
    fn add_entities_update_version() {
        let mut grid = GameGrid::new(1, 1);
        grid.add_bomb(Bomb::new(0, (0, 0), 3, 1));
        assert_eq!(grid.version(), 1);
        grid.add_agent(AgentState::new(0, (0, 0)));
        assert_eq!(grid.version(), 2);
    }
}
