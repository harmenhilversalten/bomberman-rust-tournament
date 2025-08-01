### State Crate (`bomberman_state`)

#### Module Structure
```
bomberman_state/
├── lib.rs                 // Crate public interface
├── state/                 // Core state management
│   ├── mod.rs
│   ├── game_state.rs      // Main game state structure
│   ├── entity.rs          // Entity definitions and storage
│   └── snapshot.rs        // State snapshot management
├── grid/                  // Spatial grid management
│   ├── mod.rs
│   ├── grid.rs            // Grid data structure
│   ├── cell.rs            // Cell types and states
│   └── iterator.rs        // Grid iteration patterns
├── components/            // Entity components
│   ├── mod.rs
│   ├── position.rs        // Position component
│   ├── bomb.rs            // Bomb component
│   ├── player.rs          // Player component
│   └── destructible.rs    // Destructible wall component
├── serialization/         // State serialization
│   ├── mod.rs
│   ├── encoder.rs         // State encoding
│   └── decoder.rs         // State decoding
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── state_tests.rs
    └── serialization_tests.rs
```

#### Core Data Structures

```rust
// Main game state structure
pub struct GameState {
    tick: u64,
    rng: StdRng,
    grid: Grid,
    entities: EntityStorage,
    version: AtomicU64,
    snapshots: SnapshotManager,
}

// Entity storage with archetype-based ECS
pub struct EntityStorage {
    entities: SlotMap<EntityId, EntityLocation>,
    archetypes: HashMap<ArchetypeId, Archetype>,
    component_indices: HashMap<ComponentId, usize>,
}

// Grid representation optimized for cache locality
pub struct Grid {
    width: u16,
    height: u16,
    cells: Box<[Cell]>,
    spatial_index: SpatialHashMap,
}

// Cell representation with packed data for efficiency
#[repr(C, packed)]
pub struct Cell {
    flags: CellFlags,
    entity_ids: [Option<EntityId>; 4], // Limited entities per cell
    danger: u8,                        // Danger level for influence maps
    reserved: u8,                     // Padding for alignment
}

// Entity identifier with generation for safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

// Position component with grid coordinates
#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
    subpixel_x: u8,  // For smooth movement
    subpixel_y: u8,
}

// Bomb component with timing and properties
#[derive(Debug, Clone)]
pub struct Bomb {
    owner: EntityId,
    timer: u8,
    power: u8,
    pierce: bool,
    remote: bool,
}

// Player component with bot-specific data
#[derive(Debug, Clone)]
pub struct Player {
    bot_id: BotId,
    health: u8,
    power: u8,
    speed: u8,
    bomb_count: u8,
    max_bombs: u8,
    kick: bool,
    punch: bool,
}
```

#### Key Algorithms

1. **State Delta Application**
   - Apply deltas in batch to minimize cache misses
   - Use dirty flags to identify changed regions
   - Parallelize independent updates where possible

```rust
impl GameState {
    pub fn apply_deltas(&mut self, deltas: &[StateDelta]) -> Result<(), StateError> {
        // Sort deltas by type for batch processing
        let sorted_deltas = self.sort_deltas_by_type(deltas);
        
        // Process each type of delta in batch
        for delta_type in DeltaType::iter() {
            let batch = sorted_deltas.get(&delta_type);
            if let Some(batch) = batch {
                self.apply_delta_batch(batch)?;
            }
        }
        
        // Update version after all deltas applied
        self.version.fetch_add(1, Ordering::Release);
        
        Ok(())
    }
    
    fn apply_delta_batch(&mut self, deltas: &[StateDelta]) -> Result<(), StateError> {
        match deltas.first() {
            Some(StateDelta::EntityCreated { .. }) => self.create_entities(deltas),
            Some(StateDelta::EntityDestroyed { .. }) => self.destroy_entities(deltas),
            Some(StateDelta::ComponentChanged { .. }) => self.update_components(deltas),
            Some(StateDelta::GridCellChanged { .. }) => self.update_grid_cells(deltas),
            _ => Ok(()),
        }
    }
}
```

2. **Snapshot Creation**
   - Copy-on-write snapshot creation
   - Version tracking for consistency
   - Memory-efficient storage of deltas

```rust
impl SnapshotManager {
    pub fn create_snapshot(&self, state: &GameState) -> SnapshotHandle {
        let version = state.version.load(Ordering::Acquire);
        
        // Check if we already have a snapshot for this version
        if let Some(handle) = self.snapshots.get(&version) {
            return *handle;
        }
        
        // Create new snapshot
        let snapshot = Box::new(Snapshot::new(state, version));
        let handle = SnapshotHandle::new(self.next_handle.fetch_add(1, Ordering::Relaxed));
        
        // Store snapshot with version
        self.snapshots.insert(version, handle);
        self.snapshot_data.insert(handle, snapshot);
        
        // Clean up old snapshots if needed
        self.cleanup_old_snapshots();
        
        handle
    }
}
```

#### Performance Optimizations

1. **Archetype-based ECS**: Store components in contiguous memory arrays grouped by archetype for cache efficiency.

2. **Spatial Hashing**: Use a spatial hash map for fast spatial queries instead of a traditional grid.

3. **Entity Generation**: Use generation counters in entity IDs to safely reuse slots without ABA problems.

4. **Batch Processing**: Process state deltas in batches to minimize cache misses.

5. **Snapshot Compression**: Compress snapshots using delta encoding against previous states.

#### API Design

```rust
// Main public interface for the state crate
pub struct GameStateBuilder {
    width: u16,
    height: u16,
    seed: u64,
    player_count: u8,
}

impl GameStateBuilder {
    pub fn new() -> Self { /* ... */ }
    
    pub fn width(mut self, width: u16) -> Self { /* ... */ }
    
    pub fn height(mut self, height: u16) -> Self { /* ... */ }
    
    pub fn seed(mut self, seed: u64) -> Self { /* ... */ }
    
    pub fn player_count(mut self, count: u8) -> Self { /* ... */ }
    
    pub fn build(self) -> Result<GameState, StateError> { /* ... */ }
}

impl GameState {
    /// Create a new game state with default settings
    pub fn new() -> Result<Self, StateError> {
        GameStateBuilder::new().build()
    }
    
    /// Get a snapshot of the current state
    pub fn snapshot(&self) -> SnapshotHandle {
        self.snapshots.create_snapshot(self)
    }
    
    /// Apply a delta to the game state
    pub fn apply_delta(&mut self, delta: StateDelta) -> Result<(), StateError> {
        self.apply_deltas(&[delta])
    }
    
    /// Get the current state version
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Acquire)
    }
    
    /// Query entities with specific components
    pub fn query<Q: Query>(&self) -> QueryIter<Q> {
        Q::iter(&self.entities)
    }
    
    /// Get grid cell at position
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        self.grid.get_cell(x, y)
    }
    
    /// Serialize state for transmission
    pub fn serialize(&self, format: SerializationFormat) -> Result<Vec<u8>, SerializationError> {
        match format {
            SerializationFormat::Binary => self.serialize_binary(),
            SerializationFormat::Json => self.serialize_json(),
        }
    }
}
```

#### Error Handling Strategy

```rust
/// Errors that can occur in state operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    /// Entity not found
    EntityNotFound(EntityId),
    /// Component not found for entity
    ComponentNotFound(EntityId, ComponentId),
    /// Invalid position
    InvalidPosition(u16, u16),
    /// Serialization error
    Serialization(SerializationError),
    /// Version mismatch
    VersionMismatch(u64, u64),
    /// Out of memory
    OutOfMemory,
    /// Invalid state transition
    InvalidTransition(String),
    /// I/O error
    Io(String),
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::EntityNotFound(id) => write!(f, "Entity not found: {:?}", id),
            StateError::ComponentNotFound(id, comp_id) => {
                write!(f, "Component {:?} not found for entity {:?}", comp_id, id)
            }
            StateError::InvalidPosition(x, y) => write!(f, "Invalid position: ({}, {})", x, y),
            StateError::Serialization(err) => write!(f, "Serialization error: {}", err),
            StateError::VersionMismatch(expected, actual) => {
                write!(f, "Version mismatch: expected {}, actual {}", expected, actual)
            }
            StateError::OutOfMemory => write!(f, "Out of memory"),
            StateError::InvalidTransition(msg) => write!(f, "Invalid state transition: {}", msg),
            StateError::Io(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}
```

#### Testing Strategy

1. **Unit Tests**
   - Test individual components and data structures
   - Verify state transitions and delta application
   - Validate serialization and deserialization

2. **Integration Tests**
   - Test state management with mock engine
   - Verify snapshot consistency
   - Test concurrent access patterns

3. **Property-Based Tests**
   - Generate random state sequences
   - Verify invariants are maintained
   - Test serialization round-trips

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_entity_creation() {
        let mut state = GameState::new().unwrap();
        let entity_id = state.create_entity();
        
        assert!(state.entity_exists(entity_id));
    }
    
    proptest! {
        #[test]
        fn test_serialization_roundtrip(
            width in 16u16..256,
            height in 16u16..256,
            seed in any::<u64>(),
            player_count in 1u8..8u8
        ) {
            let state = GameStateBuilder::new()
                .width(width)
                .height(height)
                .seed(seed)
                .player_count(player_count)
                .build()
                .unwrap();
                
            let serialized = state.serialize(SerializationFormat::Binary).unwrap();
            let deserialized = GameState::deserialize(&serialized, SerializationFormat::Binary).unwrap();
            
            // Verify states are equivalent
            assert_eq!(state.width(), deserialized.width());
            assert_eq!(state.height(), deserialized.height());
            assert_eq!(state.tick(), deserialized.tick());
        }
    }
}
```

