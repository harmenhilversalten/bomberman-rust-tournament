use goals::goal::{Action, AvoidDangerGoal, CollectPowerUpGoal, Goal, GoalType};
use state::GameState;

#[test]
fn goal_types_and_priorities() {
    let state = GameState::new(1, 1);
    let bot_id: goals::goal::BotId = 0;

    let collect = CollectPowerUpGoal;
    let avoid = AvoidDangerGoal;

    assert_eq!(collect.get_goal_type(), GoalType::CollectPowerUp);
    assert_eq!(avoid.get_goal_type(), GoalType::AvoidDanger);
    assert!(collect.get_priority(&state, bot_id) > avoid.get_priority(&state, bot_id));

    let plan = collect.plan(&state, bot_id).unwrap();
    assert_eq!(plan, vec![Action::Wait]);
}
