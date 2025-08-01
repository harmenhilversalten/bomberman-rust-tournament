## 12. Revised Crate Layout

### 12.1 Workspace Structure and Crate Dependencies

The project is organized as a Cargo workspace, promoting modularity, independent compilation, and clear dependency management. The root of the workspace contains a Cargo.toml file that lists all member crates. The primary crates are located within a crates/ directory. These include:

- state/ : Immutable game-state data structures.
- engine/ : Main game loop & physics.
- influence/ : Danger map calculations.
- path/ : Pathfinding algorithms (A*, D* Lite).
- goals/ : Goal generation, management, and state evaluation/scoring.
- bombs/ : Bomb-related logic and safety checks.
- bot/ : The bot kernel, configuration, and example bots.
- events/ : Game event system and broadcasting.
- rl/ : New crate for RL policies, environments, and buffers.
- test_utils/ : Utilities for testing, such as mock objects.
- ffi/ : (optional) For Foreign Function Interface bindings.

Additional directories at the root include docs/ for architecture documentation and UML diagrams, benches/ for Criterion benchmarks, and tests/ for integration tests. This structure ensures that each component has a well-defined scope and API, reducing compile times and improving code organization. Dependencies between crates are carefully managed to avoid cyclic dependencies, ensuring a clean and maintainable build process.

```
bomberman_ai/
├── Cargo.toml
├── crates/
│   ├── state/          # Immutable game-state data structures
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── grid.rs
│   │   │   └── snapshot.rs
│   ├── engine/         # Main game loop & physics
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── tick.rs
│   ├── influence/
│   ├── path/
│   ├── goals/
│   ├── bombs/
│   ├── bot/
│   ├── events/
│   ├── rl/
│   ├── test_utils/
│   └── ffi/
├── docs/
├── benches/
└── tests/
```

### 12.2 Cargo Workspace Configuration

The root Cargo.toml file defines the workspace and its common dependencies. This configuration ensures that all crates within the workspace use consistent versions of external libraries, managed by Cargo's dependency resolver (resolver = "2"). Key dependencies include tokio for asynchronous runtime, crossbeam for concurrent data structures and epoch-based memory reclamation, criterion for benchmarking, proptest for property-based testing, ndarray for N-dimensional array operations, petgraph for graph data structures, and triomphe for Arc variants. For error handling, anyhow is included. A crucial addition for the RL components is the tch crate (assuming a Rust binding for PyTorch like tch-rs), which will enable loading and running neural network models directly within Rust. This centralized dependency management simplifies builds and ensures compatibility across the entire project.

```toml
[workspace]
members = [
    "crates/state",
    "crates/engine",
    "crates/influence",
    "crates/path",
    "crates/goals",
    "crates/bombs",
    "crates/bot",
    "crates/events",
    "crates/rl",
    "crates/test_utils",
    "crates/ffi",
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"
criterion = "0.3"
proptest = "1.0"
ndarray = "0.15"
petgraph = "0.6"
triomphe = "0.1"
anyhow = "1.0"
tch = "0.7"  # For PyTorch bindings
```

### 12.3 Dependency Flow Between Crates

Dependencies flow from lower-level crates (e.g., state, engine) to higher-level ones (e.g., bot, rl). For example:
- All crates depend on state for GameGrid and snapshots.
- Engine depends on state.
- Influence, path, goals, bombs depend on state and engine (for deltas).
- Bot depends on state, engine, influence, path, goals, bombs, events.
- Rl depends on state, engine, bot (for policy integration).
- Test_utils depends on most crates for mocks.

This acyclic dependency graph ensures clean compilation and modularity.

---

