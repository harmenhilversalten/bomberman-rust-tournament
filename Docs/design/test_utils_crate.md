### Test Utils Crate (`bomberman_test_utils`)

#### Module Structure
```
bomberman_test_utils/
├── lib.rs                 // Crate public interface
├── fixtures/              // Test fixtures
│   ├── mod.rs
│   ├── game_state.rs      // Game state fixtures
│   ├── bot.rs             // Bot fixtures
│   └── scenario.rs        // Scenario fixtures
├── mocks/                 // Mock implementations
│   ├── mod.rs
│   ├── mock_ai.rs         // Mock AI
│   ├── mock_policy.rs     // Mock policy
│   └── mock_influence.rs  // Mock influence map
├── assertions/            // Custom assertions
│   ├── mod.rs
│   ├── state_assertions.rs // State assertions
│   ├── path_assertions.rs  // Path assertions
│   └── performance_assertions.rs // Performance assertions
├── generators/            // Test data generators
│   ├── mod.rs
│   ├── state_generator.rs // State generators
│   ├── bot_generator.rs   // Bot generators
│   └── scenario_generator.rs // Scenario generators
└── benchmarks/            // Benchmark utilities
    ├── mod.rs
    ├── benchmark.rs       // Benchmark runner
    └── metrics.rs         // Performance metrics
```

#### Core Data Structures
```rust
// Test scenario
pub struct TestScenario {
    name: String,
    description: String,
    initial_state: GameState,
    expected_outcomes: Vec<ExpectedOutcome>,
    steps: Vec<TestStep>,
}

// Expected outcome
pub enum ExpectedOutcome {
    StateCondition(Box<dyn Fn(&GameState) -> bool>),
    BotCondition(BotId, Box<dyn Fn(&BotState) -> bool>),
    PerformanceCondition(Box<dyn Fn(&PerformanceMetrics) -> bool>),
}

// Test step
pub struct TestStep {
    description: String,
    actions: Vec<(BotId, Action)>,
    expected_state_changes: Vec<StateChange>,
}

// Performance metrics
pub struct PerformanceMetrics {
    decision_times: HashMap<BotId, Vec<Duration>>,
    memory_usage: HashMap<BotId, usize>,
    frame_times: Vec<Duration>,
    event_processing_times: Vec<Duration>,
}
```

#### Key Algorithms
```rust
impl TestRunner {
    pub fn run_scenario(&mut self, scenario: &TestScenario) -> TestResult {
        let mut result = TestResult::new(scenario.name.clone());
        
        // Initialize game state
        let mut state = scenario.initial_state.clone();
        
        // Run test steps
        for (step_index, step) in scenario.steps.iter().enumerate() {
            // Execute actions
            for (bot_id, action) in &step.actions {
                let bot = self.bots.get_mut(bot_id)
                    .ok_or_else(|| TestError::BotNotFound(*bot_id))?;
                
                let action_result = bot.execute_action(&mut state, action.clone())
                    .map_err(|e| TestError::ActionExecutionFailed(*bot_id, e))?;
                
                result.add_action_result(*bot_id, action.clone(), action_result);
            }
            
            // Check expected state changes
            for expected_change in &step.expected_state_changes {
                if !expected_change.verify(&state) {
                    result.add_failure(TestFailure::StateChangeMismatch(
                        step_index,
                        expected_change.description(),
                    ));
                }
            }
        }
        
        // Check final outcomes
        for outcome in &scenario.expected_outcomes {
            match outcome {
                ExpectedOutcome::StateCondition(condition) => {
                    if !condition(&state) {
                        result.add_failure(TestFailure::FinalStateConditionFailed);
                    }
                }
                ExpectedOutcome::BotCondition(bot_id, condition) => {
                    if let Some(bot) = self.bots.get(bot_id) {
                        if !condition(&bot.state) {
                            result.add_failure(TestFailure::BotConditionFailed(*bot_id));
                        }
                    } else {
                        result.add_failure(TestFailure::BotNotFound(*bot_id));
                    }
                }
                ExpectedOutcome::PerformanceCondition(condition) => {
                    if !condition(&self.performance_metrics) {
                        result.add_failure(TestFailure::PerformanceConditionFailed);
                    }
                }
            }
        }
        
        result
    }
}
```

#### Performance Optimizations
1. **Parallel Test Execution**: Run multiple tests in parallel
2. **Test Caching**: Cache test results for unchanged scenarios
3. **Selective Testing**: Only run affected tests
4. **Memory Profiling**: Track memory usage during tests
5. **Benchmark Integration**: Integrate benchmarks with tests

