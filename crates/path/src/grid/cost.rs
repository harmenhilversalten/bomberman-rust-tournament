//! Movement cost calculations.

use crate::Point;

use super::PathGrid;

/// Returns the movement cost from `from` to `to` on the given `grid`.
///
/// The base cost is the destination node's cost.
pub fn movement_cost(grid: &PathGrid, _from: Point, to: Point) -> u32 {
    grid.node_cost(to)
}
