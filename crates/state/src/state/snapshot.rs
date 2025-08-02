use triomphe::Arc;

use crate::{
    components::{AgentState, Bomb},
    grid::Tile,
};

/// Inner snapshot data stored atomically.
#[derive(Debug)]
pub(crate) struct SnapshotInner {
    pub tiles: Arc<[Tile]>,
    pub bombs: Arc<[Bomb]>,
    pub agents: Arc<[AgentState]>,
    pub version: u64,
}

impl SnapshotInner {
    pub fn new(
        tiles: Arc<[Tile]>,
        bombs: Arc<[Bomb]>,
        agents: Arc<[AgentState]>,
        version: u64,
    ) -> Self {
        Self {
            tiles,
            bombs,
            agents,
            version,
        }
    }
}

/// Immutable view of the game state.
#[derive(Debug, Clone)]
pub struct SnapshotView {
    inner: Arc<SnapshotInner>,
}

impl SnapshotView {
    pub(crate) fn new(inner: Arc<SnapshotInner>) -> Self {
        Self { inner }
    }

    /// Tiles of the snapshot.
    pub fn tiles(&self) -> &[Tile] {
        &self.inner.tiles
    }

    /// Bombs present in the snapshot.
    pub fn bombs(&self) -> &[Bomb] {
        &self.inner.bombs
    }

    /// Agents present in the snapshot.
    pub fn agents(&self) -> &[AgentState] {
        &self.inner.agents
    }

    /// Version of the grid this snapshot represents.
    pub fn version(&self) -> u64 {
        self.inner.version
    }
}
