use std::sync::{Arc, Mutex};

use goals::{
    executor::{GoalExecutor, ProgressMonitor},
    goal::{Action, BotId, Goal, GoalError, GoalType},
    hierarchy::{GoalDependency, GoalHierarchy},
    planner::{GoalPlanner, PlanningStrategy},
};
use state::GameState;

#[derive(Clone)]
struct DummyGoal {
    goal_type: GoalType,
    priority: f32,
    progress: Arc<Mutex<f32>>,
    complete_at: f32,
}

impl Goal for DummyGoal {
    fn get_goal_type(&self) -> GoalType {
        self.goal_type
    }

    fn get_priority(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        self.priority
    }

    fn is_achievable(&self, _state: &GameState, _bot_id: BotId) -> bool {
        true
    }

    fn get_progress(&self, _state: &GameState, _bot_id: BotId) -> f32 {
        *self.progress.lock().unwrap()
    }

    fn is_completed(&self, _state: &GameState, _bot_id: BotId) -> bool {
        *self.progress.lock().unwrap() >= self.complete_at
    }

    fn plan(&self, _state: &GameState, _bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        Ok(vec![Action::Wait])
    }
}

#[test]
fn progress_monitor_detects_stall() {
    let mut monitor = ProgressMonitor::new(1);
    monitor.update(0.0);
    monitor.update(0.0);
    assert!(monitor.is_stalled());
}

#[test]
fn hierarchy_executes_in_dependency_order() {
    let state = GameState::new(1, 1);
    let bot_id: BotId = 0;
    let child_progress = Arc::new(Mutex::new(0.0));
    let parent_progress = Arc::new(Mutex::new(0.0));

    let child_goal = DummyGoal {
        goal_type: GoalType::AvoidDanger,
        priority: 1.0,
        progress: child_progress.clone(),
        complete_at: 1.0,
    };
    let parent_goal = DummyGoal {
        goal_type: GoalType::CollectPowerUp,
        priority: 2.0,
        progress: parent_progress.clone(),
        complete_at: 1.0,
    };

    let mut hierarchy = GoalHierarchy::default();
    hierarchy.add_goal(Box::new(child_goal.clone()), GoalDependency::default());
    hierarchy.add_goal(
        Box::new(parent_goal.clone()),
        GoalDependency::with([GoalType::AvoidDanger]),
    );

    let mut planner = GoalPlanner::new(PlanningStrategy::HighestScore);
    for node in hierarchy.next_ready() {
        planner.add_goal(node.goal.clone());
    }

    let goal = planner.select_goal(&state, bot_id).unwrap().unwrap();
    planner.activate_goal(goal, &state, bot_id, 0).unwrap();
    let mut executor = GoalExecutor::new(ProgressMonitor::new(2));
    let actions = executor.execute(&mut planner, &state, bot_id).unwrap();
    assert_eq!(actions, vec![Action::Wait]);
    *child_progress.lock().unwrap() = 1.0;
    let actions = executor.execute(&mut planner, &state, bot_id).unwrap();
    assert!(actions.is_empty());
    hierarchy.mark_completed(GoalType::AvoidDanger);

    for node in hierarchy.next_ready() {
        planner.add_goal(node.goal.clone());
    }
    let goal = planner.select_goal(&state, bot_id).unwrap().unwrap();
    planner.activate_goal(goal, &state, bot_id, 1).unwrap();
    let actions = executor.execute(&mut planner, &state, bot_id).unwrap();
    assert_eq!(actions, vec![Action::Wait]);
    *parent_progress.lock().unwrap() = 1.0;
    let actions = executor.execute(&mut planner, &state, bot_id).unwrap();
    assert!(actions.is_empty());
}
