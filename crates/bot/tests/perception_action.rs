use bot::action::{Action, ActionExecutor, ActionResult};
use bot::perception::PerceptionSystem;

#[test]
fn perception_system_records_observations() {
    let mut sys = PerceptionSystem::new();
    sys.update(1);
    sys.update(2);
    assert_eq!(sys.memory().len(), 2);
    assert_eq!(sys.memory().last().unwrap().value, 2);
}

#[test]
fn action_executor_updates_state() {
    let exec = ActionExecutor::new();
    let mut state = 0;
    let res = exec.execute(&mut state, Action::Move(3));
    assert_eq!(res, ActionResult::Success);
    assert_eq!(state, 3);
}
