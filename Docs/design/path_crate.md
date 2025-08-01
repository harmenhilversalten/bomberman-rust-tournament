### Path Crate (`bomberman_path`)

#### Module Structure
```
bomberman_path/
├── lib.rs                 // Crate public interface
├── algorithms/            // Pathfinding algorithms
│   ├── mod.rs
│   ├── astar.rs           // A* implementation
│   ├── dstar_lite.rs      // D* Lite for dynamic environments
│   └── jps.rs             // Jump Point Search
├── grid/                  // Grid representation
│   ├── mod.rs
│   ├── path_grid.rs       // Pathfinding grid
│   ├── node.rs            // Grid node
│   └── cost.rs            // Movement cost calculation
├── heuristic/             // Heuristic functions
│   ├── mod.rs
│   ├── manhattan.rs       // Manhattan distance
│   ├── euclidean.rs       // Euclidean distance
│   └── custom.rs          // Custom heuristics
├── cache/                 // Path caching
│   ├── mod.rs
│   ├── path_cache.rs      // Path cache
│   ├── cache_policy.rs    // Eviction policies
│   └── cache_key.rs       // Cache key generation
├── optimization/          // Path optimization
│   ├── mod.rs
│   ├── smoothing.rs       // Path smoothing
│   ├── funnel.rs          // Funnel algorithm
│   └── simplification.rs  // Path simplification
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── algorithm_tests.rs
    └── cache_tests.rs
```

#### Core Data Structures
```rust
// Main pathfinding structure
pub struct PathFinder {
    grid: PathGrid,
    algorithm: Box<dyn PathfindingAlgorithm>,
    heuristic: Box<dyn Heuristic>,
    cache: PathCache,
    config: PathConfig,
}

// Pathfinding grid
pub struct PathGrid {
    width: u16,
    height: u16,
    nodes: Box<[Node]>,
    blocked: Box<[bool]>,
    influence_map: Option<Arc<InfluenceMap>>,
}

// A* algorithm implementation
pub struct AStar {
    open_set: BinaryHeap<OpenSetNode>,
    closed_set: HashSet<GridPosition>,
    nodes: Vec<Node>,
    max_iterations: u32,
    early_exit_threshold: f32,
}

// Path cache
pub struct PathCache {
    cache: HashMap<CacheKey, CachedPath>,
    max_size: usize,
    eviction_policy: EvictionPolicy,
    hit_count: u64,
    miss_count: u64,
}
```

#### Key Algorithms
```rust
impl AStar {
    pub fn find_path(
        &mut self,
        grid: &PathGrid,
        start: Position,
        goal: Position,
    ) -> Result<Vec<Position>, PathError> {
        let start_grid = GridPosition::new(start.x, start.y);
        let goal_grid = GridPosition::new(goal.x, goal.y);
        
        // Initialize open set with start node
        self.open_set.clear();
        self.closed_set.clear();
        
        let start_node = Node {
            x: start_grid.x,
            y: start_grid.y,
            g_cost: 0.0,
            h_cost: self.heuristic.calculate(start_grid, goal_grid),
            f_cost: 0.0,
            parent: None,
            in_open_set: true,
            in_closed_set: false,
        };
        
        self.open_set.push(OpenSetNode::new(start_grid, start_node.f_cost));
        
        // Main loop
        let mut iterations = 0;
        while !self.open_set.is_empty() && iterations < self.max_iterations {
            iterations += 1;
            
            // Get node with lowest f_cost
            let current = self.open_set.pop().unwrap();
            let current_node = &mut self.nodes[current.position.y as usize * grid.width as usize + current.position.x as usize];
            
            // Check if we reached the goal
            if current.position == goal_grid {
                return self.reconstruct_path(current.position, start_grid, goal_grid);
            }
            
            // Move current node from open to closed set
            current_node.in_open_set = false;
            current_node.in_closed_set = true;
            self.closed_set.insert(current.position);
            
            // Process neighbors
            for neighbor_pos in grid.get_neighbors(current.position) {
                // Skip if neighbor is in closed set
                if self.closed_set.contains(&neighbor_pos) {
                    continue;
                }
                
                // Calculate tentative g_cost
                let tentative_g_cost = current_node.g_cost + grid.get_movement_cost(current.position, neighbor_pos);
                
                let neighbor_node = &mut self.nodes[neighbor_pos.y as usize * grid.width as usize + neighbor_pos.x as usize];
                
                // If neighbor not in open set or new path is better
                if !neighbor_node.in_open_set || tentative_g_cost < neighbor_node.g_cost {
                    // Update neighbor node
                    neighbor_node.g_cost = tentative_g_cost;
                    neighbor_node.h_cost = self.heuristic.calculate(neighbor_pos, goal_grid);
                    neighbor_node.f_cost = neighbor_node.g_cost + neighbor_node.h_cost;
                    neighbor_node.parent = Some(current.position);
                    
                    // Add to open set if not already there
                    if !neighbor_node.in_open_set {
                        neighbor_node.in_open_set = true;
                        self.open_set.push(OpenSetNode::new(neighbor_pos, neighbor_node.f_cost));
                    }
                }
            }
        }
        
        // No path found
        Err(PathError::NoPathFound(start, goal))
    }
}
```

#### Performance Optimizations
1. **Early Termination**: Stop when close enough to goal
2. **Path Caching**: Cache frequently used paths
3. **Path Optimization**: Smooth and simplify paths
4. **Binary Heaps**: Efficient priority queues
5. **Incremental Updates**: Update paths when grid changes slightly

