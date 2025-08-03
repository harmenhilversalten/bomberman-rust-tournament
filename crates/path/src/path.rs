//! Path structures and movement conversion.

use crate::Point;

/// Movement actions derived from a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Move up.
    Up,
    /// Move down.
    Down,
    /// Move left.
    Left,
    /// Move right.
    Right,
}

/// Node within a path.
#[derive(Debug, Clone, Copy)]
pub struct PathNode {
    /// Position of the node.
    pub position: Point,
}

/// Sequence of nodes representing a path.
#[derive(Debug, Clone)]
pub struct Path {
    /// Ordered nodes of the path.
    pub nodes: Vec<PathNode>,
}

impl Path {
    /// Create a new [`Path`] from nodes.
    pub fn new(nodes: Vec<PathNode>) -> Self {
        Self { nodes }
    }

    /// Convert the path into movement commands.
    pub fn to_movement_commands(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for w in self.nodes.windows(2) {
            let from = w[0].position;
            let to = w[1].position;
            let dx = to.x - from.x;
            let dy = to.y - from.y;
            let action = if dx > 0 {
                Action::Right
            } else if dx < 0 {
                Action::Left
            } else if dy > 0 {
                Action::Down
            } else {
                Action::Up
            };
            actions.push(action);
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_converts_to_actions() {
        let nodes = vec![
            PathNode {
                position: Point::new(0, 0),
            },
            PathNode {
                position: Point::new(1, 0),
            },
            PathNode {
                position: Point::new(1, 1),
            },
        ];
        let path = Path::new(nodes);
        let actions = path.to_movement_commands();
        assert_eq!(actions, vec![Action::Right, Action::Down]);
    }
}
