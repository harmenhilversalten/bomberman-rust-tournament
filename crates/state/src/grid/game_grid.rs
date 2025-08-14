//! Core game grid storing tiles and entities.
#![allow(unsafe_code)]
use std::sync::atomic::{AtomicU64, Ordering};

use super::{delta::GridDelta, tile::Tile};
use crate::components::{AgentState, Bomb};
use crate::state::snapshot::{SnapshotInner, SnapshotView};
use crossbeam_epoch::{self as epoch, Atomic, Owned};
use tokio::sync::watch;
use triomphe::Arc;

/// Main game grid structure holding tiles and entities.
#[derive(Debug)]
pub struct GameGrid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    bombs: Vec<Bomb>,
    agents: Vec<AgentState>,
    version: AtomicU64,
    snapshot: Atomic<SnapshotInner>,
    delta_tx: watch::Sender<GridDelta>,
}

/// Difference between two observations.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObservationDelta {
    /// Difference of tile encodings between snapshots.
    pub tiles: Vec<f32>,
}

impl GameGrid {
    /// Creates a new grid filled with `Tile::Empty` tiles.
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![Tile::Empty; width * height];
        let bombs = Vec::new();
        let agents = Vec::new();
        let (tx, _rx) = watch::channel(GridDelta::None);

        let inner = SnapshotInner::new(
            Arc::<[Tile]>::from(tiles.clone()),
            Arc::<[Bomb]>::from(bombs.clone()),
            Arc::<[AgentState]>::from(agents.clone()),
            0,
        );

        let snapshot = Atomic::new(inner);

        Self {
            width,
            height,
            tiles,
            bombs,
            agents,
            version: AtomicU64::new(0),
            snapshot,
            delta_tx: tx,
        }
    }

    /// Constructs a grid from raw parts used during deserialization.
    pub(crate) fn from_parts(
        width: usize,
        height: usize,
        tiles: Vec<Tile>,
        bombs: Vec<Bomb>,
        agents: Vec<AgentState>,
        version: u64,
    ) -> Self {
        let (tx, _rx) = watch::channel(GridDelta::None);
        let inner = SnapshotInner::new(
            Arc::<[Tile]>::from(tiles.clone()),
            Arc::<[Bomb]>::from(bombs.clone()),
            Arc::<[AgentState]>::from(agents.clone()),
            version,
        );
        Self {
            width,
            height,
            tiles,
            bombs,
            agents,
            version: AtomicU64::new(version),
            snapshot: Atomic::new(inner),
            delta_tx: tx,
        }
    }

    /// Width of the grid.
    pub(crate) fn width(&self) -> usize {
        self.width
    }

    /// Height of the grid.
    pub(crate) fn height(&self) -> usize {
        self.height
    }

    /// All tiles in the grid.
    pub(crate) fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// All bombs currently in the grid.
    pub fn bombs(&self) -> &[Bomb] {
        &self.bombs
    }

    /// All agents currently in the grid (mutable).
    pub fn agents_mut(&mut self) -> &mut [AgentState] {
        &mut self.agents
    }

    /// All agents currently in the grid.
    pub fn agents(&self) -> &[AgentState] {
        &self.agents
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

    /// Check if a bomb can be placed at `position`.
    pub fn can_place_bomb(&self, position: (u16, u16)) -> bool {
        matches!(
            self.tile(position.0 as usize, position.1 as usize),
            Some(Tile::Empty)
        )
    }

    /// Place a bomb at `position` if possible.
    pub fn place_bomb(&mut self, position: (u16, u16)) {
        self.add_bomb(Bomb::new(0, position, 3, 1));
    }

    /// Adds an agent to the grid and returns its identifier.
    pub fn add_agent(&mut self, agent: AgentState) -> usize {
        self.agents.push(agent);
        self.version.fetch_add(1, Ordering::Relaxed);
        self.agents.len() - 1
    }

    /// Applies a delta to the grid and broadcasts the change.
    pub fn apply_delta(&mut self, delta: GridDelta) {
        match &delta {
            GridDelta::None => {}
            GridDelta::SetTile { x, y, tile } => self.set_tile(*x, *y, *tile),
            GridDelta::AddBomb(b) => {
                self.bombs.push(b.clone());
                self.version.fetch_add(1, Ordering::Relaxed);
            }
            GridDelta::AddAgent(a) => {
                self.agents.push(a.clone());
                self.version.fetch_add(1, Ordering::Relaxed);
            }
            GridDelta::MoveAgent(agent_id, new_pos) => {
                if let Some(agent) = self.agents.iter_mut().find(|a| a.id == *agent_id) {
                    agent.position = *new_pos;
                    self.version.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        self.update_snapshot();
        let _ = self.delta_tx.send(delta);
    }

    /// Current version of the grid.
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }

    /// Subscribe to grid deltas.
    pub fn subscribe(&self) -> watch::Receiver<GridDelta> {
        self.delta_tx.subscribe()
    }

    /// Produce an immutable snapshot of the grid.
    pub fn snapshot(&self) -> SnapshotView {
        let guard = epoch::pin();
        let shared = self.snapshot.load(Ordering::Acquire, &guard);
        // Safety: pointer was constructed from a valid SnapshotInner
        let inner = unsafe { shared.deref() };
        let view = SnapshotView::new(Arc::new(SnapshotInner::new(
            inner.tiles.clone(),
            inner.bombs.clone(),
            inner.agents.clone(),
            inner.version,
        )));
        drop(guard);
        view
    }

    /// Serialize a snapshot into a vector of floats for RL agents.
    pub fn to_observation(&self, agent_id: usize) -> Vec<f32> {
        let snapshot = self.snapshot();
        let mut obs: Vec<f32> = snapshot.tiles().iter().map(|t| t.to_u8() as f32).collect();
        if let Some(agent) = snapshot.agents().iter().find(|a| a.id == agent_id) {
            obs.push(agent.position.0 as f32);
            obs.push(agent.position.1 as f32);
            obs.push(agent.bombs_left as f32);
            obs.push(agent.power as f32);
        } else {
            obs.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
        }
        obs
    }

    /// Generate an incremental observation compared to a previous snapshot.
    pub fn observe_delta(&self, prev: &SnapshotView) -> ObservationDelta {
        let current = self.snapshot();
        let curr_tiles: Vec<f32> = current.tiles().iter().map(|t| t.to_u8() as f32).collect();
        let prev_tiles: Vec<f32> = prev.tiles().iter().map(|t| t.to_u8() as f32).collect();
        let tiles = curr_tiles
            .iter()
            .zip(prev_tiles.iter())
            .map(|(c, p)| c - p)
            .collect();
        ObservationDelta { tiles }
    }

    fn update_snapshot(&mut self) {
        let new_inner = SnapshotInner::new(
            Arc::<[Tile]>::from(self.tiles.clone()),
            Arc::<[Bomb]>::from(self.bombs.clone()),
            Arc::<[AgentState]>::from(self.agents.clone()),
            self.version(),
        );
        let guard = epoch::pin();
        let old = self
            .snapshot
            .swap(Owned::new(new_inner), Ordering::AcqRel, &guard);
        unsafe {
            guard.defer_destroy(old);
        }
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

    #[test]
    fn snapshot_consistency() {
        let mut grid = GameGrid::new(2, 2);
        let snap = grid.snapshot();
        grid.apply_delta(GridDelta::SetTile {
            x: 0,
            y: 0,
            tile: Tile::Wall,
        });
        assert_eq!(snap.tiles()[0], Tile::Empty);
        let new_snap = grid.snapshot();
        assert_eq!(new_snap.tiles()[0], Tile::Wall);
    }

    #[test]
    fn subscribe_receives_delta() {
        let mut grid = GameGrid::new(1, 1);
        let mut rx = grid.subscribe();
        grid.apply_delta(GridDelta::SetTile {
            x: 0,
            y: 0,
            tile: Tile::Wall,
        });
        assert_eq!(
            rx.borrow_and_update().clone(),
            GridDelta::SetTile {
                x: 0,
                y: 0,
                tile: Tile::Wall
            }
        );
    }

    #[test]
    fn observe_delta_reports_changes() {
        let mut grid = GameGrid::new(2, 2);
        let prev = grid.snapshot();
        grid.apply_delta(GridDelta::SetTile {
            x: 0,
            y: 0,
            tile: Tile::Wall,
        });
        let delta = grid.observe_delta(&prev);
        assert_eq!(
            delta.tiles[0],
            Tile::Wall.to_u8() as f32 - Tile::Empty.to_u8() as f32
        );
    }

    #[test]
    fn can_place_bomb_checks_empty_tile() {
        let grid = GameGrid::new(1, 1);
        assert!(grid.can_place_bomb((0, 0)));
    }

    #[test]
    fn place_bomb_adds_bomb() {
        let mut grid = GameGrid::new(1, 1);
        grid.place_bomb((0, 0));
        assert_eq!(grid.bombs().len(), 1);
    }
}
