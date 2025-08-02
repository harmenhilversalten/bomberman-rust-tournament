//! Bomb kicking mechanics.

use std::collections::HashSet;

use crate::bomb::entity::{Bomb, Position};

/// Directions in which a bomb can be kicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Move one tile up.
    Up,
    /// Move one tile down.
    Down,
    /// Move one tile left.
    Left,
    /// Move one tile right.
    Right,
}

/// Attempts to kick `bomb` one tile in `dir`.
/// Returns `true` if the bomb was moved.
pub fn kick_bomb(
    bomb: &mut Bomb,
    dir: Direction,
    size: (u16, u16),
    walls: &HashSet<Position>,
) -> bool {
    if !bomb.kickable {
        return false;
    }
    let (mut x, mut y) = (bomb.position.0 as i32, bomb.position.1 as i32);
    match dir {
        Direction::Up => y -= 1,
        Direction::Down => y += 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
    }
    if x < 0 || y < 0 || x >= size.0 as i32 || y >= size.1 as i32 {
        return false;
    }
    let new_pos = (x as u16, y as u16);
    if walls.contains(&new_pos) {
        return false;
    }
    bomb.position = new_pos;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bomb::entity::{Bomb, BombId};

    #[test]
    fn kick_moves_kickable_bomb() {
        let mut bomb = Bomb::new(BombId(1), 0, (1, 1), 3, 1);
        bomb.kickable = true;
        assert!(kick_bomb(
            &mut bomb,
            Direction::Right,
            (5, 5),
            &HashSet::new()
        ));
        assert_eq!(bomb.position, (2, 1));
    }

    #[test]
    fn kick_blocked_by_wall() {
        let mut bomb = Bomb::new(BombId(1), 0, (1, 1), 3, 1);
        bomb.kickable = true;
        let mut walls = HashSet::new();
        walls.insert((2, 1));
        assert!(!kick_bomb(&mut bomb, Direction::Right, (5, 5), &walls));
        assert_eq!(bomb.position, (1, 1));
    }
}
