### Goals Crate (`bomberman_goals`)

#### Module Structure
```
bomberman_goals/
├── lib.rs                 // Crate public interface
├── goal/                  // Goal definitions
│   ├── mod.rs
│   ├── goal.rs            // Goal trait
│   ├── goal_types.rs      // Specific goal types
│   └── priority.rs        // Goal priority calculation
├── planner/               // Goal planning
│   ├── mod.rs
│   ├── goal_planner.rs    // Main planner
│   ├── strategy.rs        // Planning strategies
│   └── evaluation.rs      // Goal evaluation
├── executor/              // Goal execution
│   ├── mod.rs
│   ├── executor.rs        // Goal executor
│   ├── monitor.rs         // Progress monitoring
│   └── adaptation.rs      // Goal adaptation
├── hierarchy/             // Goal hierarchy
│   ├── mod.rs
│   ├── hierarchy.rs       // Goal hierarchy
│   ├── decomposition.rs   // Goal decomposition
│   └── dependency.rs      // Goal dependencies
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── goal_tests.rs
    └── planner_tests.rs
```

#### Core Data Structures
```rust
// Goal trait
pub trait Goal: Send + Sync {
    fn get_goal_type(&self) -> GoalType;
    fn get_priority(&self, state: &GameState, bot_id: BotId) -> f32;
    fn is_achievable(&self, state: &GameState, bot_id: BotId) -> bool;
    fn get_progress(&self, state: &GameState, bot_id: BotId) -> f32;
    fn is_completed(&self, state: &GameState, bot_id: BotId) -> bool;
    fn plan(&self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError>;
}

// Goal planner
pub struct GoalPlanner {
    goals: Vec<Box<dyn Goal>>,
    active_goal: Option<ActiveGoal>,
    strategy: PlanningStrategy,
    evaluation_weights: HashMap<GoalType, f32>,
}

// Active goal with execution state
pub struct ActiveGoal {
    goal: Box<dyn Goal>,
    plan: Vec<Action>,
    current_step: usize,
    start_tick: u64,
    progress: f32,
}
```

#### Key Algorithms
```rust
impl GoalPlanner {
    pub fn select_goal(&mut self, state: &GameState, bot_id: BotId) -> Result<Option<Box<dyn Goal>>, GoalError> {
        // Evaluate all goals
        let mut goal_scores: Vec<(f32, &Box<dyn Goal>)> = self.goals
            .iter()
            .filter(|goal| goal.is_achievable(state, bot_id))
            .map(|goal| {
                let priority = goal.get_priority(state, bot_id);
                let weight = self.evaluation_weights.get(&goal.get_goal_type()).unwrap_or(&1.0);
                (priority * weight, goal)
            })
            .collect();
        
        // Sort by score (descending)
        goal_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return the highest scoring goal
        if let Some((_, goal)) = goal_scores.first() {
            Ok(Some((*goal).clone()))
        } else {
            Ok(None)
        }
    }
    
    pub fn execute_active_goal(&mut self, state: &GameState, bot_id: BotId) -> Result<Vec<Action>, GoalError> {
        if let Some(ref mut active_goal) = self.active_goal {
            // Check if goal is completed
            if active_goal.goal.is_completed(state, bot_id) {
                self.active_goal = None;
                return Ok(vec![]);
            }
            
            // Update progress
            active_goal.progress = active_goal.goal.get_progress(state, bot_id);
            
            // Get next action from plan
            if active_goal.current_step < active_goal.plan.len() {
                let action = active_goal.plan[active_goal.current_step].clone();
                active_goal.current_step += 1;
                return Ok(vec![action]);
            } else {
                // Plan completed, generate new plan
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
```

#### Performance Optimizations
1. **Goal Caching**: Cache goal evaluations
2. **Lazy Planning**: Only generate plans when needed
3. **Priority Queues**: Efficient goal selection
4. **Incremental Progress**: Track progress incrementally
5. **Plan Reuse**: Reuse plans when possible

