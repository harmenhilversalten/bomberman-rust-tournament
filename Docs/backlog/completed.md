# Completed Backlog Items

This archive lists backlog items that have been completed and moved out of the active backlog.

## 1. Restructure into Workspace
- **Summary**: Convert the repository into a Cargo workspace with crates as outlined in Section 12.1 of the architecture document.
- **Requirements**:
  - Create `crates/` directory with sub‑crates `state`, `engine`, `influence`, `path`, `goals`, `bombs`, `bot`, `events`, `rl`, `test_utils`, and optional `ffi`.
  - Root `Cargo.toml` must define workspace members and shared `[workspace.dependencies]` per Section 12.2.
  - All crates except `ffi` use `#![forbid(unsafe_code)]`.
  - Include initial skeleton source files and unit test modules.
- **Prompt**: "Restructure project into a workspace as described in Architecture.md. Add minimal lib.rs for each crate and adjust existing code to compile."  Ensure tests exist for crate creation.

## 2. State Crate – Core Structures
- **Summary**: Implement the main state structures as outlined in [design/state_crate.md](../design/state_crate.md) lines 1‑27 and Architecture Section 5.1.
- **Requirements**:
  - Define `GameGrid`, `Tile`, `Bomb`, `AgentState` and version tracking with `AtomicU64`.
  - Implement grid and component modules per the design document.
  - Unit tests verify grid updates and entity handling.
- **Prompt**: "Create the base modules for bomberman_state with SOLID organization and tests for grid operations." 

## 3. State Crate – Snapshot Layer
- **Summary**: Provide immutable snapshot views of the state as in Architecture Section 5.3.
- **Requirements**:
  - Implement `SnapshotView` using `crossbeam-epoch` and triomphe `Arc`.
  - Expose `apply_delta`, `subscribe` and `to_observation` methods.
  - Tests cover snapshot consistency and observation serialization.
- **Prompt**: "Extend bomberman_state with snapshot API and tests verifying snapshots and deltas."

## 4. State Crate – Serialization
- **Summary**: Add state serialization modules following the layout in [design/state_crate.md](../design/state_crate.md).
- **Requirements**:
  - Encode and decode game state to binary/JSON formats.
  - Ensure compatibility with the replay system.
  - Unit tests validate round‑trip serialization.
- **Prompt**: "Implement state serialization/deserialization with comprehensive tests."

## 5. Engine Crate – Core Loop
- **Summary**: Create `bomberman_engine` with the main game loop as described in Architecture Section 5.2 and [design/engine_crate.md](../design/engine_crate.md) lines 1‑31.
- **Requirements**:
  - Own shared `Arc<RwLock<GameGrid>>` and broadcast `GridDelta` via `tokio::sync::watch`.
  - Provide `Engine::new()` and `Engine::tick()`.
  - Unit tests simulate ticks and assert deltas are produced.
- **Prompt**: "Implement engine crate running the game tick and emitting GridDelta events using Tokio."

## 6. Engine Crate – Scheduler
- **Summary**: Implement the task scheduler module referenced in [design/engine_crate.md](../design/engine_crate.md) lines 1‑31.
- **Requirements**:
  - Build a `TaskScheduler` for parallel system execution.
  - Integrate with the main loop to run systems in dependency order.
  - Tests verify task ordering and parallel safety.
- **Prompt**: "Add scheduler supporting parallel systems with unit tests."

## 7. Engine Crate – System Modules
- **Summary**: Add system implementations (movement, bombs, explosions, powerups, players) per the design document.
- **Requirements**:
  - Create modules under `systems/` for each subsystem.
  - Define a common `System` trait and hook into the scheduler.
  - Unit tests cover system interaction on a small grid.
- **Prompt**: "Implement engine systems and validate via tests."

## 8. Engine Crate – Replay and Determinism
- **Summary**: Provide replay recording and determinism checks as required by Architecture Section 9.2 and [design/engine_crate.md](../design/engine_crate.md) lines 1‑31.
- **Requirements**:
  - Record GridDeltas for replay and load them back for verification.
  - Implement a determinism checker that records state hashes each tick.
  - Unit tests ensure replays reproduce identical states.
- **Prompt**: "Implement replay system and determinism checks with tests."

## 9. Engine Crate – Configuration
- **Summary**: Expose engine configuration and game rules using the `config/` modules defined in [design/engine_crate.md](../design/engine_crate.md) lines 1‑31.
- **Requirements**:
  - Define `EngineConfig` and `GameRules` structures.
  - Allow loading configuration from files or defaults.
  - Unit tests verify config parsing and application.
- **Prompt**: "Add configuration support to the engine with accompanying tests."

## 10. Events Crate – Event Types and Bus
- **Summary**: Implement event definitions and the main event bus following Architecture Section 5.10 and [design/events_crate.md](../design/events_crate.md) lines 1‑29.
- **Requirements**:
  - Define `GameEvent` variants plus bot and system events.
  - Implement `EventBus` with subscriber registration.
  - Tests ensure events are broadcast to subscribers.
- **Prompt**: "Create event definitions and core bus with unit tests."

## 11. Events Crate – Queue and Filtering
- **Summary**: Add priority queues and subscription filters based on [design/events_crate.md](../design/events_crate.md) lines 1‑29.
- **Requirements**:
  - Provide `EventQueue` structures with priority levels.
  - Implement `EventFilter` for selective delivery.
  - Tests validate ordering and filtering logic.
- **Prompt**: "Extend events crate with queue management and filtering tests."

## 12. Events Crate – Serialization and Recording
- **Summary**: Support JSON serialization and RL transition recording as outlined in the design document.
- **Requirements**:
  - Implement encoder and decoder modules.
  - Generate RL `Transition` records for learning.
  - Unit tests cover serialization round trips.
- **Prompt**: "Add serialization and recording utilities with tests."

## 13. Influence Map Crate – Core Map
- **Summary**: Create the influence map data structures as described in Architecture Section 5.9 and [design/influence_map_crate.md](../design/influence_map_crate.md).
- **Requirements**:
  - Implement `InfluenceMap` with multiple layers and dirty region tracking.
  - Expose APIs for querying danger and opportunity values.
  - Unit tests verify influence propagation.
- **Prompt**: "Build base influence map modules with tests for propagation."

## 14. Influence Map Crate – Update Strategies
- **Summary**: Provide incremental and full update strategies from the design document.
- **Requirements**:
  - Implement modules under `update/` for different strategies.
  - Integrate with the core map to mark dirty regions.
  - Property tests ensure updates maintain correctness.
- **Prompt**: "Implement update strategies with property-based tests."

## 15. Influence Map Crate – Visualization and Benchmarking
- **Summary**: Add visualization helpers and Criterion benchmarks for performance.
- **Requirements**:
  - Implement rendering and export modules.
  - Benchmarks measure update times and memory usage.
  - Tests verify visualization output format.
- **Prompt**: "Provide visualization and benchmarking for influence maps."

## 16. Path Crate – Algorithm Implementations
- **Summary**: Implement pathfinding algorithms (A*, D* Lite, Jump Point Search) referencing Architecture Section 5.7 and [design/path_crate.md](../design/path_crate.md).
- **Requirements**:
  - Provide modules under `algorithms/` for each algorithm.
  - Use Manhattan heuristic influenced by `InfluenceMap`.
  - Unit tests verify path correctness on sample grids.
- **Prompt**: "Add pathfinding algorithms with thorough tests."

## 17. Path Crate – Grid and Heuristics
- **Summary**: Add grid representations and heuristic functions from the design document.
- **Requirements**:
  - Implement `PathGrid`, node structures and cost calculation.
  - Provide Manhattan and Euclidean heuristics.
  - Tests cover heuristic calculations and grid setup.
- **Prompt**: "Implement path grid and heuristics with unit tests."

## 18. Path Crate – Caching and Optimization
- **Summary**: Provide caching and path optimization modules.
- **Requirements**:
  - Implement path cache with eviction policies.
  - Add smoothing and simplification algorithms.
  - Benchmarks ensure caching improves performance.
- **Prompt**: "Optimize pathfinding with caching and benchmarks."

## 19. Bombs Crate – Bomb Logic
- **Summary**: Implement core bomb structures as in Architecture Section 5.8 and [design/bombs_crate.md](../design/bombs_crate.md).
- **Requirements**:
  - Define bomb data types and chain reaction logic.
  - Calculate blast radii using graph algorithms.
  - Unit tests verify bomb chaining and explosions.
- **Prompt**: "Create bomb management modules with tests."

## 20. Bombs Crate – Placement and Timing
- **Summary**: Add placement strategies and timer management based on the design document.
- **Requirements**:
  - Implement safe and strategic placement modules.
  - Provide timer and remote detonation support.
  - Tests cover placement decisions and timing.
- **Prompt**: "Implement bomb placement and timing with unit tests."

## 21. Bombs Crate – Power and Analysis
- **Summary**: Provide power calculation and danger/opportunity analysis tools.
- **Requirements**:
  - Implement modules for bomb power effects and kicking mechanics.
  - Add analysis utilities to query safe tiles and opportunities.
  - Property tests validate danger calculations.
- **Prompt**: "Extend bombs crate with power and analysis features."

## 22. Goals Crate – Goal Definitions and Planner
- **Summary**: Implement goal generation and scoring using Architecture Sections 5.5 and 5.6 and [design/goals_crate.md](../design/goals_crate.md).
- **Requirements**:
  - Define goal traits and specific goal types.
  - Implement the goal planner and evaluation strategies.
  - Unit tests verify goal ordering and scoring.
- **Prompt**: "Create goal definitions and planner with tests."

## 23. Goals Crate – Execution and Hierarchy
- **Summary**: Add goal execution, monitoring and hierarchy management.
- **Requirements**:
  - Implement executor modules with progress monitoring.
  - Provide hierarchy and dependency resolution.
  - Tests cover goal execution flow.
- **Prompt**: "Implement goal execution and hierarchy with unit tests."

## 24. Bot Crate – Core Kernel
- **Summary**: Provide the bot kernel coordinating decision making per Architecture Section 5.4 and [design/bot_crate.md](../design/bot_crate.md).
- **Requirements**:
  - Expose `Bot` struct with configuration options.
  - Integrate with engine via channels.
  - Unit tests simulate a bot making decisions from mock snapshots.
- **Prompt**: "Implement bot kernel with tests for the decision loop."

## 25. Bot Crate – AI Modules
- **Summary**: Implement heuristic, reactive and planning AI modules.
- **Requirements**:
  - Create modules under `ai/` for different strategies.
  - Allow dynamic switching between AI types.
  - Tests verify each AI module produces valid commands.
- **Prompt**: "Add multiple AI strategies with comprehensive tests."

## 26. Bot Crate – Perception and Action
- **Summary**: Add perception processing and action execution modules from the design document.
- **Requirements**:
  - Implement observation processing and memory handling.
  - Provide action executor and feedback handling.
  - Unit tests cover perception updates and action effects.
- **Prompt**: "Implement perception and action modules with tests."

## 27. RL Crate – Policy and Value Estimation
- **Summary**: Centralize reinforcement learning utilities as described in Architecture Section 5.11 and [design/rl_crate.md](../design/rl_crate.md).
- **Requirements**:
  - Define `Policy` and `ValueEstimator` traits with Torch-based implementations.
  - Provide basic random policies for testing.
  - Unit tests verify policy loading and inference.
- **Prompt**: "Create policy and value modules with unit tests."

## 28. RL Crate – Environment and Training
- **Summary**: Provide environment wrappers and training utilities.
- **Requirements**:
  - Implement Gym-compatible environment and reward calculation.
  - Provide training loops and replay buffers.
  - Tests run a minimal episode using a dummy policy.
- **Prompt**: "Implement RL environment and training utilities with tests."

## 29. Engine Integration of Feature Crates
- **Summary**: Replace legacy engine modules with the new crates (`bombs`, `goals`, `path`, `influence`, `events`, `bot`, `rl`) so the game loop works through the unified architecture.
- **Requirements**:
  - Use the `events` crate to broadcast `GameEvent` messages that drive influence-map updates, goal replanning, and RL rewards.
  - Feed snapshots, goals, pathfinding, and bomb planning through the newly implemented crates, removing parallel legacy code.
  - Ensure the integration honors the data-flow design where the engine updates shared state and downstream components react via deltas and events.
- **Prompt**: "Wire up all feature crates in the engine and retire overlapping legacy modules."
