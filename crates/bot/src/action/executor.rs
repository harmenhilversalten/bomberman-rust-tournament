//! Executes actions against the game grid.

use super::{Action, ActionResult};
use state::grid::GameGrid;

/// Trait for applying an [`Action`] to a game grid.
pub trait ActionExecutor {
    /// Execute the action against the grid and return the result.
    fn execute(&self, grid: &mut GameGrid) -> ActionResult;
}

impl ActionExecutor for Action {
    fn execute(&self, grid: &mut GameGrid) -> ActionResult {
        match self {
            Action::PlaceBomb { position } => {
                if grid.can_place_bomb(*position) {
                    grid.place_bomb(*position);
                    ActionResult::Success
                } else {
                    ActionResult::Failure("cannot place bomb here")
                }
            }
            Action::Move(_) => ActionResult::Success,
            Action::Wait => ActionResult::Success,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::grid::{GameGrid, Tile};

    #[test]
    fn placing_bomb_success() {
        let action = Action::PlaceBomb { position: (0, 0) };
        let mut grid = GameGrid::new(1, 1);
        assert!(matches!(action.execute(&mut grid), ActionResult::Success));
    }

    #[test]
    fn placing_bomb_fails_when_occupied() {
        let action = Action::PlaceBomb { position: (0, 0) };
        let mut grid = GameGrid::new(1, 1);
        grid.set_tile(0, 0, Tile::Wall);
        assert!(matches!(
            action.execute(&mut grid),
            ActionResult::Failure(_)
        ));
    }
}
