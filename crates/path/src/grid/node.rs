//! Grid node representation.

/// Represents a single grid cell.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// Whether the cell can be traversed.
    pub walkable: bool,
    /// Base movement cost (1 = normal).
    pub cost: u32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            walkable: true,
            cost: 1,
        }
    }
}
