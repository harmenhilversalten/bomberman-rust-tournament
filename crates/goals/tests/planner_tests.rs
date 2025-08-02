use goals::{
    goal::{Action, AvoidDangerGoal, CollectPowerUpGoal, GoalType},
    planner::{GoalPlanner, PlanningStrategy},
};
use state::GameState;

#[test]
fn planner_selects_highest_scoring_goal() {
    let state = GameState::new(1, 1);
    let bot_id: goals::goal::BotId = 0;
    let mut planner = GoalPlanner::new(PlanningStrategy::HighestScore);
    planner.add_goal(Box::new(AvoidDangerGoal));
    planner.add_goal(Box::new(CollectPowerUpGoal));

    let selected = planner.select_goal(&state, bot_id).unwrap().unwrap();
    assert_eq!(selected.get_goal_type(), GoalType::CollectPowerUp);
}

#[test]
fn planner_executes_active_goal_plan() {
    let state = GameState::new(1, 1);
    let bot_id: goals::goal::BotId = 0;
    let mut planner = GoalPlanner::new(PlanningStrategy::HighestScore);
    planner
        .activate_goal(Box::new(CollectPowerUpGoal), &state, bot_id, 0)
        .unwrap();

    let actions = planner.execute_active_goal(&state, bot_id).unwrap();
    assert_eq!(actions, vec![Action::Wait]);
}
