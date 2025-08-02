//! Find safe tiles and potential opportunities from bombs.

use std::collections::HashSet;

use crate::bomb::entity::{Bomb, Position};

/// Returns all tiles safe from any bomb.
pub fn safe_tiles(size: (u16, u16), walls: &HashSet<Position>, bombs: &[Bomb]) -> Vec<Position> {
    let danger = super::danger::danger_tiles(bombs, size, walls);
    let mut tiles = Vec::new();
    for x in 0..size.0 {
        for y in 0..size.1 {
            let pos = (x, y);
            if !walls.contains(&pos) && !danger.contains(&pos) {
                tiles.push(pos);
            }
        }
    }
    tiles
}

/// Returns target positions that are hit by any bomb.
pub fn opportunity_tiles(
    targets: &[Position],
    bombs: &[Bomb],
    size: (u16, u16),
    walls: &HashSet<Position>,
) -> Vec<Position> {
    let danger = super::danger::danger_tiles(bombs, size, walls);
    targets
        .iter()
        .copied()
        .filter(|p| danger.contains(p))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bomb::entity::{Bomb, BombId};

    #[test]
    fn safe_tiles_excludes_danger() {
        let bomb = Bomb::new(BombId(1), 0, (1, 1), 0, 1);
        let safe = safe_tiles((3, 3), &HashSet::new(), &[bomb]);
        assert!(!safe.contains(&(1, 1)));
        assert!(safe.contains(&(0, 0)));
    }

    #[test]
    fn opportunity_tiles_returns_hits() {
        let bomb = Bomb::new(BombId(1), 0, (0, 0), 0, 2);
        let targets = vec![(2, 0), (1, 2), (0, 2)];
        let hits = opportunity_tiles(&targets, &[bomb], (5, 5), &HashSet::new());
        assert_eq!(hits, vec![(2, 0), (0, 2)]);
    }
}
