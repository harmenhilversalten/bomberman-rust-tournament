//! Goal planner implementation handling selection and execution.

use std::collections::HashMap;

use state::GameState;

use crate::goal::{Action, BotId, Goal, GoalError, GoalType};

use super::{evaluation::evaluate_goal, strategy::PlanningStrategy};

/// Planner that evaluates goals and executes the active one.
pub struct GoalPlanner {
    goals: Vec<Box<dyn Goal>>,
    /// Currently active goal with its plan.
    pub active_goal: Option<ActiveGoal>,
    strategy: PlanningStrategy,
    evaluation_weights: HashMap<GoalType, f32>,
}

impl GoalPlanner {
    /// Creates a new planner with the given strategy.
    pub fn new(strategy: PlanningStrategy) -> Self {
        Self {
            goals: Vec::new(),
            active_goal: None,
            strategy,
            evaluation_weights: HashMap::new(),
        }
    }

    /// Adds a goal to the planner's pool.
    pub fn add_goal(&mut self, goal: Box<dyn Goal>) {
        self.goals.push(goal);
    }

    /// Sets a weight for a specific goal type used during evaluation.
    pub fn set_weight(&mut self, goal_type: GoalType, weight: f32) {
        self.evaluation_weights.insert(goal_type, weight);
    }

    /// Select the best goal according to the strategy.
    pub fn select_goal(
        &mut self,
        state: &GameState,
        bot_id: BotId,
    ) -> Result<Option<Box<dyn Goal>>, GoalError> {
        if self.goals.is_empty() {
            return Ok(None);
        }

        let mut scored: Vec<(f32, usize)> = self
            .goals
            .iter()
            .enumerate()
            .filter(|(_, g)| g.is_achievable(state, bot_id))
            .map(|(idx, g)| {
                (
                    evaluate_goal(&**g, state, bot_id, &self.evaluation_weights),
                    idx,
                )
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        match self.strategy {
            PlanningStrategy::HighestScore => {
                if let Some((_, idx)) = scored.first() {
                    Ok(Some(self.goals[*idx].clone()))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Activates the specified goal and generates its plan.
    pub fn activate_goal(
        &mut self,
        goal: Box<dyn Goal>,
        state: &GameState,
        bot_id: BotId,
        start_tick: u64,
    ) -> Result<(), GoalError> {
        let plan = goal.plan(state, bot_id)?;
        self.active_goal = Some(ActiveGoal::new(goal, plan, start_tick));
        Ok(())
    }

    /// Executes the currently active goal, returning actions to perform.
    pub fn execute_active_goal(
        &mut self,
        state: &GameState,
        bot_id: BotId,
    ) -> Result<Vec<Action>, GoalError> {
        if let Some(ref mut active_goal) = self.active_goal {
            if active_goal.goal.is_completed(state, bot_id) {
                self.active_goal = None;
                return Ok(vec![]);
            }

            active_goal.progress = active_goal.goal.get_progress(state, bot_id);

            if active_goal.current_step < active_goal.plan.len() {
                let action = active_goal.plan[active_goal.current_step].clone();
                active_goal.current_step += 1;
                return Ok(vec![action]);
            } else {
                active_goal.plan = active_goal.goal.plan(state, bot_id)?;
                active_goal.current_step = 0;

                if !active_goal.plan.is_empty() {
                    let action = active_goal.plan[active_goal.current_step].clone();
                    active_goal.current_step += 1;
                    return Ok(vec![action]);
                }
            }
        }

        Ok(vec![])
    }
}

/// Represents a goal currently being executed along with its plan state.
pub struct ActiveGoal {
    /// Goal being executed.
    pub goal: Box<dyn Goal>,
    /// Planned actions for the goal.
    pub plan: Vec<Action>,
    /// Current index in the plan.
    pub current_step: usize,
    /// Tick when the goal was started.
    pub start_tick: u64,
    /// Latest progress measurement.
    pub progress: f32,
}

impl ActiveGoal {
    /// Creates a new active goal instance.
    pub fn new(goal: Box<dyn Goal>, plan: Vec<Action>, start_tick: u64) -> Self {
        Self {
            goal,
            plan,
            current_step: 0,
            start_tick,
            progress: 0.0,
        }
    }
}
