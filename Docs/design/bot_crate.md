### Bot Crate (`bomberman_bot`)

#### Module Structure
```
bomberman_bot/
├── lib.rs                 // Crate public interface
├── bot/                   // Core bot functionality
│   ├── mod.rs
│   ├── bot.rs             // Bot structure
│   ├── decision.rs        // Decision making
│   └── state.rs           // Bot state
├── ai/                    // AI implementations
│   ├── mod.rs
│   ├── heuristic_ai.rs    // Heuristic-based AI
│   ├── reactive_ai.rs     // Reactive AI
│   └── planning_ai.rs     // Planning-based AI
├── perception/            // Bot perception
│   ├── mod.rs
│   ├── perception.rs      // Perception system
│   ├── observation.rs     // Observation processing
│   └── memory.rs          // Bot memory
├── action/                // Action execution
│   ├── mod.rs
│   ├── action.rs          // Action types
│   ├── executor.rs        // Action executor
│   └── feedback.rs       // Action feedback
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── bot_tests.rs
    └── ai_tests.rs
```

#### Core Data Structures
```rust
// Bot structure
pub struct Bot {
    id: BotId,
    entity_id: EntityId,
    ai: Box<dyn BotAI>,
    perception: PerceptionSystem,
    action_executor: ActionExecutor,
    state: BotState,
    config: BotConfig,
    decision_stats: DecisionStats,
}

// Bot AI trait
pub trait BotAI: Send + Sync {
    fn get_ai_type(&self) -> BotAIType;
    fn make_decision(&mut self, perception: &Perception, state: &BotState) -> Result<BotDecision, BotError>;
    fn on_action_result(&mut self, action: Action, result: ActionResult);
    fn get_memory_usage(&self) -> usize;
}

// Perception system
pub struct PerceptionSystem {
    observation_range: u16,
    memory: BotMemory,
    influence_map: Arc<InfluenceMap>,
    pathfinder: Arc<PathFinder>,
    bomb_manager: Arc<BombManager>,
}

// Bot state
pub struct BotState {
    health: u8,
    power: u8,
    speed: u8,
    bomb_count: u8,
    max_bombs: u8,
    kick: bool,
    punch: bool,
    last_action: Option<Action>,
    last_action_result: Option<ActionResult>,
    current_goal: Option<GoalType>,
}
```

#### Key Algorithms
```rust
impl Bot {
    pub fn make_decision(&mut self, state: &GameState) -> Result<BotDecision, BotError> {
        let start_time = Instant::now();
        
        // Update perception
        let perception = self.perception.update(state, self.entity_id)?;
        
        // Make decision using AI
        let decision = self.ai.make_decision(&perception, &self.state)?;
        
        // Record decision time
        let decision_time = start_time.elapsed();
        self.decision_stats.record_decision(decision_time);
        
        // Check decision time constraints
        if decision_time.as_millis() > 2 {
            log::warn!("Bot {} decision took {:?} (exceeds 2ms limit)", self.id, decision_time);
        }
        
        Ok(decision)
    }
    
    pub fn execute_action(&mut self, state: &mut GameState, action: Action) -> Result<ActionResult, BotError> {
        // Execute action
        let result = self.action_executor.execute(state, self.entity_id, action)?;
        
        // Update bot state
        self.update_state_after_action(action, &result);
        
        // Notify AI of action result
        self.ai.on_action_result(action, result.clone());
        
        Ok(result)
    }
    
    fn update_state_after_action(&mut self, action: Action, result: &ActionResult) {
        self.last_action = Some(action);
        self.last_action_result = Some(result.clone());
        
        // Update bot state based on action result
        match result {
            ActionResult::Success => {
                // Update state based on successful action
                if let Action::PlaceBomb { .. } = action {
                    self.bomb_count += 1;
                }
            }
            ActionResult::Failure(reason) => {
                // Handle failure
                log::debug!("Bot {} action failed: {:?}", self.id, reason);
            }
            ActionResult::Partial => {
                // Handle partial success
            }
        }
    }
}
```

#### Performance Optimizations
1. **Perception Caching**: Cache perception calculations
2. **Decision Timeouts**: Enforce strict decision time limits
3. **Lazy Evaluation**: Only calculate what's needed
4. **Memory Pooling**: Reuse memory allocations
5. **Parallel Processing**: Process independent tasks in parallel

