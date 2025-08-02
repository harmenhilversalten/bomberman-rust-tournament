//! Determine dangerous tiles from active bombs.

use std::collections::HashSet;

use crate::{
    bomb::entity::{Bomb, Position},
    power::affected_tiles,
};

/// Computes all tiles affected by any of the provided `bombs`.
pub fn danger_tiles(
    bombs: &[Bomb],
    size: (u16, u16),
    walls: &HashSet<Position>,
) -> HashSet<Position> {
    let mut danger = HashSet::new();
    for bomb in bombs {
        let tiles = affected_tiles(bomb.position, bomb.power, size, walls, bomb.pierce);
        danger.extend(tiles);
    }
    danger
}

/// Returns `true` if `pos` is not covered by any bomb blast.
pub fn is_safe(pos: Position, bombs: &[Bomb], size: (u16, u16), walls: &HashSet<Position>) -> bool {
    !danger_tiles(bombs, size, walls).contains(&pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bomb::entity::{Bomb, BombId};
    use proptest::prelude::*;

    #[test]
    fn empty_field_is_safe() {
        assert!(is_safe((0, 0), &[], (5, 5), &HashSet::new()));
    }

    proptest! {
        #[test]
        fn danger_within_power(
            x in 0u16..5,
            y in 0u16..5,
            power in 1u8..4,
        ) {
            let bomb = Bomb::new(BombId(1), 0, (x, y), 0, power);
            let tiles = danger_tiles(&[bomb.clone()], (5, 5), &HashSet::new());
            prop_assert!(tiles.contains(&bomb.position));
            for &(tx, ty) in &tiles {
                let dist = tx.abs_diff(x) + ty.abs_diff(y);
                prop_assert!(dist <= power as u16);
            }
        }
    }
}
