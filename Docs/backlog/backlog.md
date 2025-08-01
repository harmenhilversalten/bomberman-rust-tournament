# Bomberman AI Development Backlog

This backlog lists the high level features required to evolve the current project into the architecture defined in [Docs/Architecture.md](../Architecture.md).  The full architecture is decomposed under `Docs/architecture/`. Each item includes a summary, key requirements and a short prompt describing the tasks to implement.  Features are ordered roughly by dependency so later tasks can rely on the preceding work.

## 1. Restructure into Workspace
- **Summary**: Convert the repository into a Cargo workspace with crates as outlined in Section 12.1 of the architecture document.
- **Requirements**:
  - Create `crates/` directory with sub‑crates `state`, `engine`, `influence`, `path`, `goals`, `bombs`, `bot`, `events`, `rl`, `test_utils`, and optional `ffi`.
  - Root `Cargo.toml` must define workspace members and shared `[workspace.dependencies]` per Section 12.2.
  - All crates except `ffi` use `#![forbid(unsafe_code)]`.
  - Include initial skeleton source files and unit test modules.
- **Prompt**: "Restructure project into a workspace as described in Architecture.md. Add minimal lib.rs for each crate and adjust existing code to compile."  Ensure tests exist for crate creation.

## 2. State Crate
- **Summary**: Implement `bomberman_state` crate providing immutable game state and snapshot layer as in Section 5.1 and 5.3.
- **Requirements**:
  - Define `GameGrid`, `Tile`, `Bomb`, `AgentState` and version tracking using `AtomicU64`.
  - Provide `apply_delta`, `subscribe` and `to_observation` methods.
  - Implement `SnapshotView` for lock‑free reads using `crossbeam-epoch` and triomphe `Arc`.
  - Unit tests cover snapshot consistency and observation serialization.
- **Prompt**: "Create bomberman_state crate with grid representation and snapshot API. Follow SOLID principles and include tests verifying snapshots and deltas."  

## 3. Engine Crate
- **Summary**: Add `bomberman_engine` crate containing the main game loop and tick logic (Section 5.2).
- **Requirements**:
  - Own shared `Arc<RwLock<GameGrid>>` and broadcast `GridDelta` via `tokio::sync::watch`.
  - Provide `Engine::new()` and `Engine::tick()`.
  - Unit tests simulate a few ticks and assert deltas are produced.
- **Prompt**: "Implement engine crate running the game tick and emitting GridDelta events."  Use async features from Tokio and write tests.

## 4. Events Crate
- **Summary**: Implement asynchronous event system for game events (Section 5.10).
- **Requirements**:
  - Define `GameEvent` enum and broadcast using `tokio::sync::broadcast`.
  - Support episode serialization to JSON and generation of RL `Transition` records.
  - Tests cover event broadcast and serialization.
- **Prompt**: "Add events crate with GameEvent enum and broadcast/record logic."  Ensure unit tests validate event delivery.

## 5. Influence Map Crate
- **Summary**: Create `influence` crate producing danger maps (Section 5.9).
- **Requirements**:
  - Maintain `InfluenceMap` with dirty tracking and incremental updates.
  - Export influence values for RL observations.
  - Benchmarks using Criterion for update performance.
  - Unit tests for incremental update logic.
- **Prompt**: "Implement influence map computations and expose to observation."  Include Criterion benchmark and tests.

## 6. Path Crate
- **Summary**: Provide pathfinding algorithms (Section 5.7).
- **Requirements**:
  - Implement A* with Manhattan heuristic influenced by `InfluenceMap`.
  - Add optional D* Lite for path repair and macro‑move expansion.
  - Unit tests verify path correctness on sample grids.
- **Prompt**: "Create path crate with A* and macro move planner."  Write comprehensive tests.

## 7. Bombs Crate
- **Summary**: Manage bomb placement logic and safety checks (Section 5.8).
- **Requirements**:
  - Calculate chain reactions using `petgraph` and provide `safe_tiles` queries using BFS.
  - Integrate with influence map for danger assessment.
  - Optional `RlBombPolicy` hook.
  - Unit tests for blast radius calculations and safety detection.
- **Prompt**: "Add bombs crate implementing bomb planner and safety helpers."  Ensure tests cover chain reactions and BFS logic.

## 8. Goals Crate
- **Summary**: Implement goal generation and scoring (Sections 5.5 and 5.6).
- **Requirements**:
  - Manage a `GoalHeap` prioritizing objectives based on scores.
  - Include a state evaluator module to assess board value.
  - Unit tests exercise goal prioritization and evaluation.
- **Prompt**: "Create goals crate producing goals from snapshots and evaluating state."  Provide tests for goal ordering.

## 9. Bot Crate
- **Summary**: Provide bot kernel coordinating decision making (Section 5.4).
- **Requirements**:
  - Expose `Bot` struct with `BotConfig` supporting RL or heuristic mode.
  - Integrate with engine via channels and call into path, bombs, goals, etc.
  - Allow loading RL policies dynamically.
  - Unit tests simulate a bot making decisions from mock snapshots.
- **Prompt**: "Implement bot crate with switchable RL and programmatic logic."  Follow SOLID design and write tests for the decision loop.

## 10. RL Crate
- **Summary**: Centralize reinforcement learning utilities (Section 5.11 and 8.3).
- **Requirements**:
  - Define `Policy` and `ValueEstimator` traits plus `TorchPolicy` implementation using `tch`.
  - Provide `BomberEnv` Gym wrapper and `PolicyRegistry` for loading models.
  - Unit tests verify policy loading and environment step/reset behavior.
- **Prompt**: "Add rl crate with TorchPolicy and BomberEnv."  Include tests that load a dummy model and run a minimal episode.

## 11. Threading & Concurrency
- **Summary**: Establish multi-threaded execution model (Section 7).
- **Requirements**:
  - Engine runs in its own thread or async task.
  - Each bot runs in a per‑bot async task receiving deltas through watch channels and sending commands via mpsc.
  - Lock‑free snapshot access using crossbeam-epoch.
  - Unit tests spawn a few bots and verify concurrent operation deterministically.
- **Prompt**: "Wire up engine and bots using async tasks and channels as described."  Ensure tests demonstrate concurrency without data races.

## 12. Test Utilities Crate
- **Summary**: Provide helpers for unit and integration testing.
- **Requirements**:
  - Mocks for snapshots and simplified game grids.
  - Utilities for generating random maps with Proptest.
  - Exposed to other crates as dev-dependency.
- **Prompt**: "Create test_utils crate with mocks and property-based test helpers."  Include sample property test.

## 13. CI Workflows
- **Summary**: Add GitHub Actions pipelines enforcing quality gates (Section 13).
- **Requirements**:
  - `ci.yml` running `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo miri test`.
  - `bench.yml` running Criterion benches on pull requests.
  - `coverage.yml` running Tarpaulin and uploading to Codecov.
- **Prompt**: "Add GitHub workflows for check, clippy, tests, miri, benchmarks and coverage."  Ensure the workflows pass locally before pushing.

## 14. Replay & Benchmarking Tools
- **Summary**: Provide replay capability and benchmarks (Section 9.2).
- **Requirements**:
  - Serialize GridDeltas and GameEvents to disk for replay.
  - Criterion benchmarks for pathfinding, influence updates and decision loop.
  - Unit tests for the replay loader.
- **Prompt**: "Implement replay recording/loading and add benchmarks."  Write tests verifying replay reconstruction.

## 15. Documentation & Examples
- **Summary**: Document how to create external crates and provide examples (Section 14).
- **Requirements**:
  - Add guide in `docs/` explaining workspace usage and sample agent crate.
  - Ensure README references Architecture and backlog.
- **Prompt**: "Write documentation showing how to depend on bomberman crates from a new project."  Include minimal example code.

## 16. Future Enhancements
- **Summary**: Items listed in the architecture roadmap (Section 11) for later phases.
- **Requirements**:
  - GPU acceleration for influence maps.
  - Multi-agent RL algorithms.
  - WebAssembly and Bevy integration.
- **Prompt**: "Track advanced roadmap items for future work."  Do not implement yet, but keep notes for expansion.

---
This backlog should be consulted whenever implementing new features.  Each feature must follow SOLID principles, maintain small focused modules and include unit tests.  Performance targets from Section 3 of the architecture document—≤1 ms median decision time and ≤16 MB memory per bot—must be respected throughout development.
