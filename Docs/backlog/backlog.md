# Bomberman AI Development Backlog

This backlog lists the high level features required to evolve the current project into the architecture defined in [Docs/Architecture.md](../Architecture.md). The full architecture is decomposed under `Docs/architecture/`. Each item includes a summary, key requirements and a short prompt describing the tasks to implement.  Features are ordered roughly by dependency so later tasks can rely on the preceding work.
Completed backlog items 1-29 are archived in [completed.md](completed.md).


## BPI-001: Integrate Event Bus Across All Components
* **Summary**: Introduce a centralized event bus so every crate communicates through it; the engine publishes `GridDelta` events after each tick and the bot replies with command events.
* **Requirements**
  * All crates must use the centralized event bus for communication
  * `GridDelta` events must be broadcast to all subscribed components
  * Bot commands must be sent via the event bus to the engine
  * Event serialization/deserialization must work across crate boundaries
* **Files that need changing**
  * `crates/engine/src/engine/game_engine.rs` – Enhance event broadcasting
  * `crates/bot/src/bot/kernel.rs` – Add event subscription and command publishing
  * `crates/events/src/lib.rs` – Implement event serialization
  * `crates/goals/src/lib.rs` – Add event subscription
  * `crates/path/src/lib.rs` – Add event subscription
  * `crates/influence/src/lib.rs` – Add event subscription
  * `crates/rl/src/lib.rs` – Add event subscription
  * `crates/bombs/src/lib.rs` – Add event subscription
* **What needs to change**
  * Engine must publish `GridDelta` events to the event bus after each tick
  * Bot kernel must subscribe to `GridDelta` events and publish command events
  * All AI components (`goals`, `path`, `influence`, `rl`) must subscribe to relevant events
  * Event bus must handle cross-crate event serialization
* **Prompt**: “Implement event bus integration across all crates. Ensure `GridDelta` events are broadcast after each engine tick and that bot commands are published back to the engine via the event bus. Add event subscriptions to all AI components.”

## BPI-006: Implement Unified Initialization and Configuration
* **Summary**: Create a single entry point that initializes every crate in the correct order, using one coherent configuration and robust error handling.
* **Requirements**
  * All crates must be initialized from a single entry point
  * Configuration must be centralized and consistent
  * Component dependencies must be resolved at startup
  * Error handling must be comprehensive
* **Files that need changing**
  * `crates/engine/src/main.rs` – Add unified initialization
  * `crates/engine/src/config/mod.rs` – Add comprehensive configuration
  * `crates/bot/src/bot/config.rs` – Extend bot configuration
  * `crates/engine/src/lib.rs` – Add initialization function
  * `crates/ffi/src/lib.rs` – Add initialization API for FFI
* **What needs to change**
  * Main application must initialize all crates in correct order
  * Configuration system must handle all component settings
  * Event bus must be started before other components
  * Bot instances must be created with proper configuration
* **Prompt**: “Implement unified initialization and configuration for all crates. Create a centralized configuration system, ensure proper initialization order of all components, and add comprehensive error handling for startup failures.”

---

## BPI-007: Implement Tournament Cycle Functionality
* **Summary**: Add a tournament server capable of running multiple games, registering bots, tracking scores, and producing aggregated results.
* **Requirements**
  * Tournament server must manage multiple game sessions
  * Bot registration and scoring must be implemented
  * Game results must be collected and aggregated
  * Tournament lifecycle management must be robust
* **Files that need changing**
  * `crates/engine/src/tournament.rs` – Implement tournament logic
  * `crates/engine/src/main.rs` – Add tournament mode
  * `crates/engine/src/config/mod.rs` – Add tournament configuration
  * `crates/engine/src/lib.rs` – Export tournament functions
  * `crates/bot/src/bot/config.rs` – Add tournament bot configuration
* **What needs to change**
  * Tournament system must manage multiple game instances
  * Bot registration and scoring must be tracked
  * Game results must be aggregated and ranked
  * Tournament lifecycle (start, run, end) must be implemented
* **Prompt**: “Implement tournament cycle functionality including multi-game session management, bot registration and scoring, result aggregation, and tournament lifecycle management. Add tournament mode to the main application.”

---

## BPI-008: Implement Proper Error Handling and Logging

* **Summary**: Establish consistent logging and graceful error handling throughout all crates to aid debugging and recovery.
* **Requirements**
  * All components must have comprehensive error handling
  * Logging must be consistent across all crates
  * Error recovery mechanisms must be implemented
  * Debug information must be available for troubleshooting
* **Files that need changing**
  * `crates/engine/src/engine/game_engine.rs` – Add error handling
  * `crates/bot/src/bot/kernel.rs` – Add error handling
  * `crates/events/src/lib.rs` – Add error handling
  * `crates/engine/src/main.rs` – Add logging setup
  * *All crate* `lib.rs` files – Add error types and logging
* **What needs to change**
  * Engine must handle system failures gracefully
  * Bot must handle decision timeouts and errors
  * Event bus must handle subscription/publishing errors
  * Logging must be configured consistently across all crates
* **Prompt**: “Implement comprehensive error handling and logging across all crates. Add graceful error recovery, consistent logging configuration, and debug information for troubleshooting. Ensure all components handle failures appropriately.”

---

## BPI-009: Add Missing System Dependencies
* **Summary**: Declare every missing dependency, configure feature flags, eliminate circular references, and align versions across the workspace.
* **Requirements**
  * All system dependencies must be explicitly declared
  * Dependency versions must be compatible
  * Feature flags must be properly configured
  * Circular dependencies must be resolved
* **Files that need changing**
  * `crates/engine/Cargo.toml` – Add missing dependencies
  * `crates/bot/Cargo.toml` – Add missing dependencies
  * `crates/goals/Cargo.toml` – Add missing dependencies
  * `crates/path/Cargo.toml` – Add missing dependencies
  * `crates/influence/Cargo.toml` – Add missing dependencies
  * `crates/rl/Cargo.toml` – Add missing dependencies
  * `crates/bombs/Cargo.toml` – Add missing dependencies
  * `Cargo.toml` (workspace) – Ensure version consistency
* **What needs to change**
  * Engine must depend on `events`, `bot`, `bombs` crates
  * Bot must depend on `goals`, `path`, `influence`, `rl` crates
  * All AI crates must depend on `events` and `state`
  * Workspace dependencies must be consistent
* **Prompt**: “Add missing system dependencies to all crates. Ensure engine depends on events, bot, and bombs; bot depends on goals, path, influence, and rl; and all AI crates depend on events and state. Resolve any circular dependencies.”

---

## BPI-010: Implement Comprehensive Testing Integration
* **Summary**: Build a test suite with integration tests, mocks, edge-case scenarios, and performance benchmarks that cover every crate interaction.
* **Requirements**
  * Integration tests must cover all crate interactions
  * Mock objects must be available for testing
  * Test scenarios must cover edge cases
  * Performance benchmarks must be established
* **Files that need changing**
  * `crates/engine/src/lib.rs` – Add integration tests
  * `crates/bot/src/lib.rs` – Add integration tests
  * `tests/` directory – Create integration test files
  * `crates/test_utils/src/lib.rs` – Add mock objects
  * *All crate* `lib.rs` files – Add test utilities
* **What needs to change**
  * Integration tests must verify event bus communication
  * Tests must verify bot-engine interaction
  * Mock objects must simulate game state and events
  * Performance tests must measure decision latency
* **Prompt**: “Implement comprehensive testing integration including integration tests for all crate interactions, mock objects for testing, edge-case coverage, and performance benchmarks. Ensure tests verify event bus communication and bot-engine interaction.”


## 31. Legacy Cleanup
- **Summary**: Remove or deprecate remaining engine code that duplicates functionality now provided by the feature crates, keeping only the legacy bot strategy for selection.
- **Requirements**:
  - Eliminate obsolete modules (e.g., old map, bomb, path, event logic) after confirming the replacements are fully wired.
  - Update documentation and tests to reflect the new crate-based architecture.
- **Prompt**: "Clean up legacy modules and docs once new integrations pass tests."

## 32. Threading & Concurrency
- **Summary**: Establish the multi-threaded execution model described in Architecture Section 7.
- **Requirements**:
  - Engine runs in its own async task.
  - Each bot runs in a per-bot task receiving deltas and sending commands.
  - Lock-free snapshots via crossbeam-epoch.
  - Tests spawn multiple bots and verify deterministic behavior.
- **Prompt**: "Wire up engine and bots using async tasks and channels with tests."

## 33. Test Utilities Crate
- **Summary**: Provide helpers for unit and integration testing as outlined in [design/test_utils_crate.md](../design/test_utils_crate.md).
- **Requirements**:
  - Offer fixtures, mocks and generators for common scenarios.
  - Include custom assertions and benchmark helpers.
  - Property tests demonstrate random map generation.
- **Prompt**: "Create test_utils crate with mocks and property-based helpers."

## 34. CI Workflows
- **Summary**: Add GitHub Actions pipelines enforcing quality gates (Architecture Section 13).
- **Requirements**:
  - `ci.yml` running `cargo check`, `cargo clippy -- -D warnings`, `cargo test` and `cargo miri test`.
  - `bench.yml` running Criterion benches on pull requests.
  - `coverage.yml` running Tarpaulin and uploading to Codecov.
- **Prompt**: "Add CI workflows and ensure they pass locally."

## 35. Replay & Benchmarking Tools
- **Summary**: Provide replay capability and benchmarks referencing Architecture Section 9.2.
- **Requirements**:
  - Serialize GridDeltas and GameEvents to disk for replay.
  - Criterion benchmarks for pathfinding, influence updates and decision loop.
  - Unit tests for the replay loader.
- **Prompt**: "Implement replay recording/loading and add benchmarks with tests."

## 36. Documentation & Examples
- **Summary**: Document how to create external crates and provide examples (Architecture Section 14).
- **Requirements**:
  - Add guide in `docs/` explaining workspace usage and sample agent crate.
  - Ensure README references Architecture and backlog.
- **Prompt**: "Write documentation showing how to depend on bomberman crates from a new project."

## 37. Future Enhancements
- **Summary**: Items listed in the architecture roadmap (Section 11) for later phases.
- **Requirements**:
  - GPU acceleration for influence maps.
  - Multi-agent RL algorithms.
  - WebAssembly and Bevy integration.
- **Prompt**: "Track advanced roadmap items for future work but do not implement yet."

---
This backlog should be consulted whenever implementing new features. Each feature must follow SOLID principles, maintain small focused modules and include unit tests. Performance targets from Section 3 of the architecture document—≤1 ms median decision time and ≤16 MB memory per bot—must be respected throughout development.
