use state::GameGrid;

/// Computes a deterministic hash of the game grid.
pub fn hash_grid(grid: &GameGrid) -> u64 {
    let snapshot = grid.snapshot();
    let mut hash = 0u64;
    for tile in snapshot.tiles() {
        hash = hash.wrapping_mul(31).wrapping_add(tile.to_u8() as u64);
    }
    for bomb in snapshot.bombs() {
        hash = hash
            .wrapping_mul(37)
            .wrapping_add(bomb.owner as u64)
            .wrapping_add(bomb.position.0 as u64)
            .wrapping_add(bomb.position.1 as u64)
            .wrapping_add(bomb.timer as u64)
            .wrapping_add(bomb.power as u64)
            .wrapping_add(if bomb.pierce { 1 } else { 0 })
            .wrapping_add(if bomb.remote { 1 } else { 0 } << 1);
    }
    for agent in snapshot.agents() {
        hash = hash
            .wrapping_mul(41)
            .wrapping_add(agent.id as u64)
            .wrapping_add(agent.position.0 as u64)
            .wrapping_add(agent.position.1 as u64)
            .wrapping_add(agent.bombs_left as u64)
            .wrapping_add(agent.power as u64);
    }
    hash
}

/// Tracks hashes for determinism verification.
#[derive(Default)]
pub struct DeterminismChecker {
    hashes: Vec<u64>,
}

impl DeterminismChecker {
    /// Create a new checker.
    pub fn new() -> Self {
        Self { hashes: Vec::new() }
    }

    /// Record the current grid state and return its hash.
    pub fn record(&mut self, grid: &GameGrid) -> u64 {
        let hash = hash_grid(grid);
        self.hashes.push(hash);
        hash
    }

    /// Access recorded hashes.
    pub fn hashes(&self) -> &[u64] {
        &self.hashes
    }
}
