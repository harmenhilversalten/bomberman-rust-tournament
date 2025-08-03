use bot::action::{Action, ActionExecutor, ActionResult};
use bot::perception::PerceptionSystem;
use state::grid::GameGrid;

#[test]
fn perception_system_records_observations() {
    let mut sys = PerceptionSystem::new();
    sys.update(1);
    sys.update(2);
    assert_eq!(sys.memory().len(), 2);
    assert_eq!(sys.memory().last().unwrap().value, 2);
}

#[test]
fn action_executor_places_bomb() {
    let action = Action::PlaceBomb { position: (0, 0) };
    let mut grid = GameGrid::new(1, 1);
    let res = action.execute(&mut grid);
    assert_eq!(res, ActionResult::Success);
}
