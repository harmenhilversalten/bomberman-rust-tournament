### RL Crate (`bomberman_rl`)

#### Module Structure
```
bomberman_rl/
├── lib.rs                 // Crate public interface
├── policy/                // Policy implementations
│   ├── mod.rs
│   ├── policy.rs          // Policy trait
│   ├── torch_policy.rs    // PyTorch policy
│   └── random_policy.rs   // Random policy
├── value/                 // Value estimation
│   ├── mod.rs
│   ├── value_estimator.rs // Value estimator trait
│   ├── torch_value.rs     // PyTorch value estimator
│   └── mcts.rs            // Monte Carlo Tree Search
├── environment/           // RL environment
│   ├── mod.rs
│   ├── env.rs             // Gym-compatible environment
│   ├── observation.rs     // Observation space
│   └── reward.rs          // Reward calculation
├── training/              // Training utilities
│   ├── mod.rs
│   ├── trainer.rs         // Training loop
│   ├── buffer.rs          // Replay buffer
│   └── collector.rs      // Data collector
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── policy_tests.rs
    └── env_tests.rs
```

#### Core Data Structures
```rust
// Policy trait
pub trait Policy: Send + Sync {
    fn get_policy_type(&self) -> PolicyType;
    fn select_action(&mut self, observation: &Observation) -> Result<Action, RLError>;
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    fn save(&self, path: &Path) -> Result<(), RLError>;
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
    fn get_memory_usage(&self) -> usize;
}

// Value estimator trait
pub trait ValueEstimator: Send + Sync {
    fn get_value(&self, observation: &Observation) -> Result<f32, RLError>;
    fn update(&mut self, batch: &TrainingBatch) -> Result<(), RLError>;
    fn save(&self, path: &Path) -> Result<(), RLError>;
    fn load(&mut self, path: &Path) -> Result<(), RLError>;
}

// RL environment
pub struct RLEnvironment {
    game_state: GameState,
    bot_id: BotId,
    reward_calculator: Box<dyn RewardCalculator>,
    observation_space: ObservationSpace,
    action_space: ActionSpace,
    episode_length: u32,
    current_step: u32,
}

// Training batch
pub struct TrainingBatch {
    observations: Vec<Observation>,
    actions: Vec<Action>,
    rewards: Vec<f32>,
    next_observations: Vec<Observation>,
    dones: Vec<bool>,
}
```

#### Key Algorithms
```rust
impl RLEnvironment {
    pub fn step(&mut self, action: Action) -> Result<(Observation, f32, bool, RLError), RLError> {
        // Execute action
        let result = self.execute_action(action)?;
        
        // Calculate reward
        let reward = self.reward_calculator.calculate_reward(
            &self.game_state,
            self.bot_id,
            action,
            &result,
        )?;
        
        // Get new observation
        let observation = self.get_observation()?;
        
        // Check if episode is done
        let done = self.is_episode_done();
        
        // Update step counter
        self.current_step += 1;
        
        Ok((observation, reward, done, RLError::None))
    }
    
    pub fn reset(&mut self) -> Result<Observation, RLError> {
        // Reset game state
        self.game_state = GameState::new()?;
        
        // Reset step counter
        self.current_step = 0;
        
        // Get initial observation
        self.get_observation()
    }
    
    fn get_observation(&self) -> Result<Observation, RLError> {
        let mut obs = Observation::new(self.observation_space.clone());
        
        // Get bot position
        let bot_entity = self.get_bot_entity()?;
        let position = self.game_state.get_component::<Position>(bot_entity)
            .ok_or(RLError::ComponentNotFound(bot_entity, ComponentId::of::<Position>()))?;
        
        // Add position to observation
        obs.add_feature("position_x", position.x as f32 / self.game_state.width() as f32);
        obs.add_feature("position_y", position.y as f32 / self.game_state.height() as f32);
        
        // Add influence map data
        if let Some(influence_map) = &self.influence_map {
            let danger = influence_map.get_influence(position.x, position.y, InfluenceType::Danger)?;
            obs.add_feature("danger", danger);
            
            let opportunity = influence_map.get_influence(position.x, position.y, InfluenceType::Opportunity)?;
            obs.add_feature("opportunity", opportunity);
        }
        
        // Add nearby entities
        self.add_nearby_entities_to_observation(&mut obs, position)?;
        
        Ok(obs)
    }
}
```

#### Performance Optimizations
1. **Batch Processing**: Process multiple transitions in batches
2. **Observation Caching**: Cache observation calculations
3. **Memory Efficient Storage**: Compact representation of observations
4. **Parallel Training**: Train multiple policies in parallel
5. **GPU Acceleration**: Utilize GPU for neural network computations

