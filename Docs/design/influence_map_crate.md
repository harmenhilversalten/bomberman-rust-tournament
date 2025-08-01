### Influence Map Crate (`bomberman_influence`)

#### Module Structure
```
bomberman_influence/
├── lib.rs                 // Crate public interface
├── core/                  // Core influence map functionality
│   ├── mod.rs
│   ├── influence_map.rs   // Main influence map structure
│   ├── layer.rs           // Influence layer abstraction
│   └── propagation.rs    // Influence propagation algorithms
├── danger/                // Danger map implementation
│   ├── mod.rs
│   ├── danger_map.rs      // Danger calculation
│   ├── bomb_influence.rs  // Bomb danger propagation
│   └── player_influence.rs // Player threat calculation
├── opportunity/           // Opportunity map implementation
│   ├── mod.rs
│   ├── opportunity_map.rs // Opportunity calculation
│   ├── powerup_map.rs     // Powerup opportunity
│   └── player_map.rs      // Player opportunity
├── update/                // Update strategies
│   ├── mod.rs
│   ├── incremental.rs     // Incremental updates
│   ├── full.rs            // Full recalculation
│   └── dirty_tracking.rs  // Dirty region tracking
├── visualization/         // Visualization helpers
│   ├── mod.rs
│   ├── renderer.rs        // Map rendering
│   └── export.rs          // Data export
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── map_tests.rs
    └── propagation_tests.rs
```

#### Core Data Structures
```rust
// Main influence map structure
pub struct InfluenceMap {
    width: u16,
    height: u16,
    layers: HashMap<InfluenceType, Box<dyn InfluenceLayer>>,
    update_strategy: Box<dyn UpdateStrategy>,
    dirty_regions: Vec<DirtyRegion>,
    last_update_tick: u64,
}

// Influence layer trait
pub trait InfluenceLayer: Send + Sync {
    fn get_influence(&self, x: u16, y: u16) -> f32;
    fn set_influence(&mut self, x: u16, y: u16, value: f32);
    fn update(&mut self, state: &GameState, dirty_regions: &[DirtyRegion]) -> Result<(), InfluenceError>;
    fn clear(&mut self);
    fn get_layer_type(&self) -> InfluenceType;
}

// Danger map implementation
pub struct DangerMap {
    width: u16,
    height: u16,
    data: Vec<f32>,
    bomb_sources: Vec<BombSource>,
    player_sources: Vec<PlayerSource>,
    decay_rate: f32,
    max_influence: f32,
}

// Opportunity map implementation
pub struct OpportunityMap {
    width: u16,
    height: u16,
    data: Vec<f32>,
    powerup_sources: Vec<PowerupSource>,
    player_sources: Vec<PlayerSource>,
    decay_rate: f32,
    max_influence: f32,
}

// Dirty region for incremental updates
pub struct DirtyRegion {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    priority: UpdatePriority,
}
```

#### Key Algorithms
```rust
impl DangerMap {
    pub fn propagate_influence(&mut self) -> Result<(), InfluenceError> {
        // Clear previous influence
        self.data.fill(0.0);
        
        // Process bomb sources
        for bomb in &self.bomb_sources {
            self.propagate_bomb_influence(bomb)?;
        }
        
        // Process player sources
        for player in &self.player_sources {
            self.propagate_player_influence(player)?;
        }
        
        Ok(())
    }
    
    fn propagate_bomb_influence(&mut self, bomb: &BombSource) -> Result<(), InfluenceError> {
        let center_x = bomb.position.x as usize;
        let center_y = bomb.position.y as usize;
        let power = bomb.power as usize;
        
        // Calculate influence based on bomb timer
        let timer_factor = 1.0 - (bomb.timer as f32 / 10.0);
        let base_influence = bomb.influence * timer_factor;
        
        // Propagate in four directions
        for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let mut current_influence = base_influence;
            
            for distance in 1..=power {
                let (x, y) = match direction {
                    Direction::Up => (center_x, center_y.saturating_sub(distance)),
                    Direction::Right => (center_x + distance, center_y),
                    Direction::Down => (center_x, center_y + distance),
                    Direction::Left => (center_x.saturating_sub(distance), center_y),
                };
                
                // Check bounds and walls
                if x >= self.width as usize || y >= self.height as usize || self.is_blocked(x as u16, y as u16) {
                    break;
                }
                
                // Apply influence with decay
                let decay = 1.0 - (distance as f32 / power as f32);
                let index = y * self.width as usize + x;
                self.data[index] = (self.data[index] + current_influence * decay).min(self.max_influence);
                
                // Reduce influence for next cell
                current_influence *= self.decay_rate;
                
                // Early termination if influence becomes negligible
                if current_influence < 0.01 {
                    break;
                }
            }
        }
        
        Ok(())
    }
}
```

#### Performance Optimizations
1. **Incremental Updates**: Only recalculate influence in dirty regions
2. **Layered Architecture**: Separate influence types for parallel processing
3. **Early Termination**: Stop propagation when influence becomes negligible
4. **Spatial Partitioning**: Efficiently find influence sources
5. **Memory Layout**: Contiguous arrays for cache efficiency

#### API Design
```rust
impl InfluenceMap {
    pub fn new(width: u16, height: u16) -> Result<Self, InfluenceError> {
        // Initialize with default layers
        let mut layers = HashMap::new();
        layers.insert(InfluenceType::Danger, Box::new(DangerMap::new(width, height)?));
        layers.insert(InfluenceType::Opportunity, Box::new(OpportunityMap::new(width, height)?));
        
        Ok(Self {
            width,
            height,
            layers,
            update_strategy: Box::new(IncrementalUpdateStrategy::new()),
            dirty_regions: Vec::new(),
            last_update_tick: 0,
        })
    }
    
    pub fn update(&mut self, state: &GameState) -> Result<(), InfluenceError> {
        // Identify dirty regions based on state changes
        self.identify_dirty_regions(state)?;
        
        // Update each layer
        for layer in self.layers.values_mut() {
            layer.update(state, &self.dirty_regions)?;
        }
        
        // Clear dirty regions
        self.dirty_regions.clear();
        self.last_update_tick = state.tick();
        
        Ok(())
    }
    
    pub fn get_influence(&self, x: u16, y: u16, layer_type: InfluenceType) -> Result<f32, InfluenceError> {
        if let Some(layer) = self.layers.get(&layer_type) {
            Ok(layer.get_influence(x, y))
        } else {
            Err(InfluenceError::LayerNotFound(layer_type))
        }
    }
}
```

