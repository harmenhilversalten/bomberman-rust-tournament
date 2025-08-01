### Bombs Crate (`bomberman_bombs`)

#### Module Structure
```
bomberman_bombs/
├── lib.rs                 // Crate public interface
├── bomb/                  // Bomb logic
│   ├── mod.rs
│   ├── bomb.rs            // Bomb structure
│   ├── chain.rs           // Chain reaction logic
│   └── explosion.rs       // Explosion calculation
├── placement/             // Bomb placement strategies
│   ├── mod.rs
│   ├── placer.rs          // Placement interface
│   ├── strategic.rs       // Strategic placement
│   └── safe.rs            // Safe placement
├── timing/                // Bomb timing
│   ├── mod.rs
│   ├── timer.rs           // Timer management
│   └── remote.rs          // Remote detonation
├── power/                 // Bomb power and effects
│   ├── mod.rs
│   ├── power_calc.rs      // Power calculation
│   ├── pierce.rs          // Piercing bombs
│   └── kick.rs            // Bomb kicking
├── analysis/              // Bomb analysis
│   ├── mod.rs
│   ├── danger.rs          // Danger analysis
│   ├── opportunity.rs     // Opportunity analysis
│   └── simulation.rs     // Bomb simulation
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── bomb_tests.rs
    └── chain_tests.rs
```

#### Core Data Structures
```rust
// Main bomb management
pub struct BombManager {
    bombs: HashMap<BombId, Bomb>,
    active_chains: Vec<BombChain>,
    explosion_queue: PriorityQueue<ExplosionEvent, u8>,
    config: BombConfig,
    state: Arc<Mutex<GameState>>,
}

// Bomb structure
pub struct Bomb {
    id: BombId,
    owner: EntityId,
    position: Position,
    timer: u8,
    power: u8,
    pierce: bool,
    remote: bool,
    kickable: bool,
    chain_id: Option<BombChainId>,
}

// Bomb chain for chain reactions
pub struct BombChain {
    id: BombChainId,
    bombs: Vec<BombId>,
    trigger_bomb: BombId,
    explosion_time: u8,
    propagation_graph: Graph<BombId, BombChainEdge>,
}
```

#### Key Algorithms
```rust
impl BombManager {
    pub fn calculate_chain_reactions(&mut self) -> Result<(), BombError> {
        // Build bomb adjacency graph
        let graph = self.build_bomb_graph()?;
        
        // Find connected components (chains)
        let chains = self.find_bomb_chains(&graph)?;
        
        // Calculate explosion timing for each chain
        for chain in chains {
            self.calculate_chain_timing(chain, &graph)?;
        }
        
        Ok(())
    }
    
    fn build_bomb_graph(&self) -> Result<Graph<BombId, BombChainEdge>, BombError> {
        let mut graph = Graph::new();
        
        // Add nodes for each bomb
        for bomb_id in self.bombs.keys() {
            graph.add_node(*bomb_id);
        }
        
        // Add edges for bombs that can trigger each other
        for (bomb_id1, bomb1) in &self.bombs {
            for (bomb_id2, bomb2) in &self.bombs {
                if bomb_id1 == bomb_id2 {
                    continue;
                }
                
                // Check if bomb1 can trigger bomb2
                if self.bombs_in_range(bomb1, bomb2) {
                    graph.add_edge(
                        *bomb_id1,
                        *bomb_id2,
                        BombChainEdge::new(bomb1.timer),
                    );
                }
            }
        }
        
        Ok(graph)
    }
    
    fn calculate_explosion(&self, bomb: &Bomb) -> Result<Explosion, BombError> {
        let mut affected_cells = Vec::new();
        let mut affected_entities = Vec::new();
        
        // Calculate explosion in each direction
        for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let (cells, entities) = self.calculate_explosion_direction(bomb, direction)?;
            affected_cells.extend(cells);
            affected_entities.extend(entities);
        }
        
        // Add the bomb's own position
        affected_cells.push(bomb.position);
        
        // Remove duplicates
        affected_cells.sort();
        affected_cells.dedup();
        affected_entities.sort();
        affected_entities.dedup();
        
        Ok(Explosion {
            bomb_id: bomb.id,
            position: bomb.position,
            power: bomb.power,
            pierce: bomb.pierce,
            affected_cells,
            affected_entities,
        })
    }
}
```

#### Performance Optimizations
1. **Graph-Based Chain Analysis**: Efficient chain reaction calculation
2. **Spatial Partitioning**: Quickly find bombs in range
3. **Batch Processing**: Process multiple bomb updates together
4. **Explosion Caching**: Cache explosion calculations
5. **Lazy Evaluation**: Only calculate chains when necessary

