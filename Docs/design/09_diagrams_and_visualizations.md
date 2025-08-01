## 1.9 Diagrams and Visualizations

### Component Interaction Diagram

```mermaid
graph TD
    A[GameState] --> B[EventBus]
    A --> C[InfluenceMap]
    A --> D[PathFinder]
    A --> E[BombManager]
    
    B --> F[Bot]
    B --> G[Engine]
    
    C --> F
    D --> F
    E --> F
    
    F --> H[ActionExecutor]
    
    G --> A
    H --> A
    
    I[RL Policy] --> F
    J[Goal Planner] --> F
```

### Data Flow Diagram

```mermaid
flowchart TD
    A[Game State] --> B[Perception System]
    B --> C[Bot AI]
    C --> D[Decision Making]
    D --> E[Action Executor]
    E --> F[Game State Updates]
    
    G[Influence Map] --> B
    H[Path Finder] --> C
    I[Bomb Manager] --> C
    
    F --> A
    F --> G
    F --> H
    F --> I
```

### Memory Layout Diagram

```
+------------------------+
|       GameState        |
+------------------------+
| - tick: u64            |
| - rng: StdRng          |
| - grid: Grid           |
| - entities: EntityStorage |
| - version: AtomicU64   |
+------------------------+

+------------------------+
|         Grid           |
+------------------------+
| - width: u16           |
| - height: u16          |
| - cells: [Cell]        |
+------------------------+

+------------------------+
|         Cell           |
+------------------------+
| - flags: CellFlags     |
| - entity_ids: [Option<EntityId>; 4] |
| - danger: u8           |
| - reserved: u8         |
+------------------------+
```

### Concurrency Model Diagram

```mermaid
graph TB
    A[Main Thread] --> B[Tokio Runtime]
    B --> C[Bot Decision Tasks]
    B --> D[State Update Tasks]
    B --> E[Event Processing Tasks]
    B --> F[Pathfinding Tasks]
    
    G[Lock-free Event Queue] --> E
    H[Lock-free Entity Storage] --> D
    I[Lock-free Snapshot Manager] --> A
    
    J[MPSC Channels] --> C
    K[Broadcast Channels] --> B
```

### Performance Critical Path Diagram

```mermaid
gantt
    title Performance Critical Path
    dateFormat  sss
    section Bot Decision Loop
    Perception Update       :a1, 0, 200
    Influence Query         :a2, after a1, 100
    Pathfinding            :a3, after a2, 300
    Goal Evaluation        :a4, after a3, 150
    Decision Making        :a5, after a4, 200
    Action Execution       :a6, after a5, 50
    
    section State Update
    Event Processing       :b1, 0, 150
    Bomb Updates           :b2, after b1, 100
    Influence Updates      :b3, after b2, 200
    Entity Updates         :b4, after b3, 100
```

### State Machine Diagrams

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Moving : Move command
    Moving --> Idle : Movement complete
    Idle --> PlacingBomb : Place bomb command
    PlacingBomb --> Idle : Bomb placed
    Idle --> KickingBomb : Kick bomb command
    KickingBomb --> Idle : Kick complete
    Idle --> Dead : Health reaches 0
    Dead --> [*]
```

