//! Calculation of tiles affected by bomb power.

use std::collections::HashSet;

use crate::bomb::entity::Position;

/// Computes the set of tiles affected by a bomb at `position` with a given `power`.
///
/// * `size` - Grid dimensions `(width, height)`.
/// * `walls` - Positions that block blast propagation.
/// * `pierce` - If true, blast continues past walls.
pub fn affected_tiles(
    position: Position,
    power: u8,
    size: (u16, u16),
    walls: &HashSet<Position>,
    pierce: bool,
) -> HashSet<Position> {
    let mut tiles = HashSet::new();
    tiles.insert(position);

    let directions = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    for (dx, dy) in directions {
        let mut x = position.0 as i32;
        let mut y = position.1 as i32;
        for _ in 0..power {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= size.0 as i32 || y >= size.1 as i32 {
                break;
            }
            let pos = (x as u16, y as u16);
            if walls.contains(&pos) {
                if pierce {
                    continue;
                } else {
                    break;
                }
            }
            tiles.insert(pos);
        }
    }

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_respects_bounds_and_walls() {
        let mut walls = HashSet::new();
        walls.insert((2, 1));
        let tiles = affected_tiles((1, 1), 3, (5, 5), &walls, false);
        assert!(tiles.contains(&(1, 4))); // down
        assert!(!tiles.contains(&(3, 1))); // blocked by wall
    }
}
