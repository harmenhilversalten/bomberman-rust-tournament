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

---

## BPI-011: Fix Incomplete Bot Kernel Integration
* **Summary**: Fix and complete the bot kernel implementation ensuring proper AI integration.
* **Requirements**
  * Complete bot kernel implementation with proper AI component integration
  * Ensure Bot struct uses AIDecisionPipeline from BPI-003
  * Connect bot decision making to event bus command publishing
  * Add proper bot lifecycle management
* **Files that need changing**
  * `crates/bot/src/bot/kernel.rs` – Complete or fix bot kernel implementation
  * `crates/bot/src/bot/mod.rs` – Ensure proper module exports
  * `crates/bot/src/lib.rs` – Verify bot kernel is properly exported
* **What needs to change**
  * Bot kernel must instantiate and use AIDecisionPipeline
  * Bot must subscribe to GridDelta events and publish commands via event bus
  * Bot configuration must include AI component settings
  * Bot lifecycle must be properly managed by engine
* **Prompt**: "Complete the bot kernel implementation by integrating the AIDecisionPipeline, connecting to event bus for state updates and command publishing, and ensuring proper bot lifecycle management. Fix any missing bot kernel functionality."

---

## BPI-012: Fix Missing SystemInitializer Implementation
* **Summary**: Finalize the SystemInitializer with validated configuration and strict ordering.
* **Requirements**
  * Complete SystemInitializer implementation with proper initialization order
  * Ensure UnifiedConfig can be loaded from files with validation
  * Implement proper error handling for initialization failures
  * Add component health checks after initialization
* **Files that need changing**
  * `crates/engine/src/config/mod.rs` – Complete SystemInitializer implementation
  * `crates/engine/src/lib.rs` – Ensure SystemInitializer is properly exported
  * `crates/engine/src/main.rs` – Verify initialization flow
* **What needs to change**
  * SystemInitializer must follow correct initialization order (event bus → state → engine → AI components → bots)
  * Configuration validation must catch inconsistencies between components
  * Error handling must provide clear diagnostics for startup failures
  * Health checks must verify all components are properly initialized
* **Prompt**: "Complete the SystemInitializer implementation with proper initialization order, comprehensive configuration validation, robust error handling, and component health checks. Ensure the unified initialization system works correctly for both single game and tournament modes."

---

## BPI-013: Fix Event Bus Serialization Issues
* **Summary**: Ensure all event types serialize correctly with proper filtering and priority.
* **Requirements**
  * Complete event serialization for all event types
  * Ensure event filtering works correctly for different component types
  * Implement proper event priority and ordering
  * Add event bus performance monitoring
* **Files that need changing**
  * `crates/events/src/lib.rs` – Complete event serialization implementation
  * `crates/events/src/bus.rs` – Fix event filtering and subscription
  * `crates/events/src/queue.rs` – Implement event priority handling
  * `crates/engine/src/engine/game_engine.rs` – Verify event usage
* **What needs to change**
  * All event types must be serializable/deserializable across crate boundaries
  * Event filters must correctly route events to appropriate subscribers
  * Event priority must ensure critical events are processed first
  * Event bus must handle high-frequency events without performance degradation
* **Prompt**: "Complete event bus serialization for all event types, fix event filtering and subscription mechanisms, implement proper event priority handling, and add performance monitoring to ensure the event bus can handle high-frequency events efficiently."

---

## BPI-014: Fix Incomplete AI Component Integration
* **Summary**: Align goals, pathfinding, and influence maps with the AI decision pipeline.
* **Requirements**
  * Ensure AI components work together seamlessly
  * Fix goal scoring to properly use influence map data
  * Implement dynamic pathfinding with danger zone consideration
  * Complete integration between AI decision pipeline and bot kernel
* **Files that need changing**
  * `crates/bot/src/ai/pipeline.rs` – Fix AI component coordination
  * `crates/goals/src/lib.rs` – Ensure goal scoring uses influence data
  * `crates/path/src/lib.rs` – Implement dynamic pathfinding
  * `crates/influence/src/lib.rs` – Fix influence map integration
  * `crates/bot/src/bot/kernel.rs` – Complete AI pipeline integration
* **What needs to change**
  * GoalManager must generate goals based on current game state and influence data
  * Pathfinder must consider dynamic obstacles and danger zones from influence maps
  * Influence maps must be updated based on game state changes via event bus
  * AIDecisionPipeline must be properly instantiated and used by Bot kernel
* **Prompt**: "Fix AI component integration by ensuring goals, pathfinding, and influence maps work together seamlessly. Implement dynamic pathfinding with danger zone consideration, fix goal scoring to use influence data, and complete the integration between AI decision pipeline and bot kernel."

---

## BPI-015: Fix RL Integration Issues
* **Summary**: Strengthen reinforcement learning integration with detailed observations and training support.
* **Requirements**
  * Complete RL observation generation with proper game state encoding
  * Implement robust RL policy loading with error handling
  * Add reward calculation and experience buffer for training
  * Fix RL mode switching and configuration
* **Files that need changing**
  * `crates/bot/src/ai/rl_ai.rs` – Complete RL observation generation
  * `crates/rl/src/lib.rs` – Add policy loading and training support
  * `crates/state/src/lib.rs` – Implement proper observation generation
  * `crates/bot/src/bot/config.rs` – Fix RL configuration options
* **What needs to change**
  * RL observations must encode full game state (tiles, agents, bombs, power-ups) from agent's perspective
  * Policy loading must handle model file errors gracefully with fallback to programmatic AI
  * Reward calculation must support training scenarios with configurable reward shaping
  * RL mode must be properly toggleable with runtime configuration changes
* **Prompt**: "Complete RL integration by implementing proper game state observation generation, robust policy loading with error handling, reward calculation for training support, and working RL mode switching. Ensure RL observations encode the full game state from the agent's perspective."

---

## BPI-016: Fix Tournament System Integration Issues
* **Summary**: Finalize tournament modules with robust bot and session management.
* **Requirements**
  * Complete all tournament module implementations
  * Ensure proper bot registration and lifecycle management in tournaments
  * Fix game session management with proper error handling
  * Add comprehensive tournament configuration validation
* **Files that need changing**
  * `crates/engine/src/tournament/game_session.rs` – Complete game session management
  * `crates/engine/src/tournament/registry.rs` – Fix bot registration
  * `crates/engine/src/tournament/scheduler.rs` – Complete tournament scheduling
  * `crates/engine/src/tournament/scoring.rs` – Fix result aggregation
  * `crates/engine/src/config/mod.rs` – Add tournament configuration validation
* **What needs to change**
  * Game sessions must properly manage bot instances and game execution
  * Bot registry must handle tournament-specific bot lifecycle management
  * Tournament scheduler must support different tournament formats (round-robin, elimination)
  * Scoring system must aggregate results correctly across multiple games
  * Configuration validation must ensure tournament settings are consistent
* **Prompt**: "Complete tournament system integration by finishing all tournament module implementations, fixing bot registration and lifecycle management, completing game session management with proper error handling, and adding comprehensive tournament configuration validation."

---

## BPI-017: Fix Error Handling Consistency Issues
* **Summary**: Standardize error management and recovery across the codebase.
* **Requirements**
  * Standardize error types across all crates
  * Complete error recovery mechanisms for all components
  * Verify circuit breaker implementation works correctly
  * Ensure proper error propagation with context preservation
* **Files that need changing**
  * `crates/common/src/lib.rs` – Standardize error types
  * `crates/engine/src/engine/game_engine.rs` – Complete error recovery
  * `crates/events/src/lib.rs` – Verify circuit breaker implementation
  * `crates/bot/src/bot/kernel.rs` – Add error handling for bot decisions
  * All crate `src/error.rs` files – Standardize error types
* **What needs to change**
  * All crates must use consistent error types from common crate
  * Engine must recover gracefully from system failures with retry mechanisms
  * Event bus circuit breaker must prevent cascading failures
  * Bot must handle decision timeouts and AI component failures
  * Error messages must include sufficient context for debugging
* **Prompt**: "Fix error handling consistency by standardizing error types across all crates, completing error recovery mechanisms, verifying circuit breaker implementation, and ensuring proper error propagation with context preservation. Add comprehensive error handling for all component failures."

---

## BPI-018: Add Missing Configuration Files and Validation
* **Summary**: Provide default configs with robust validation and overrides.
* **Requirements**
  * Create default configuration files for all deployment modes
  * Complete cross-component configuration validation
  * Fix environment variable override functionality
  * Verify feature flags work correctly for optional components
* **Files that need changing**
  * `config/default.toml` – Create default configuration
  * `config/tournament.toml` – Create tournament configuration
  * `crates/engine/src/config/mod.rs` – Complete configuration validation
  * `crates/engine/src/main.rs` – Fix environment variable handling
  * All crate `Cargo.toml` files – Verify feature flags
* **What needs to change**
  * Default configuration must include all necessary settings for development
  * Tournament configuration must include all tournament-specific settings
  * Configuration validation must catch inconsistencies between engine, bot, and AI settings
  * Environment variables must be able to override any configuration setting
  * Feature flags must properly enable/disable optional components like RL
* **Prompt**: "Add missing configuration files and validation by creating default configurations for all deployment modes, completing cross-component configuration validation, fixing environment variable override functionality, and verifying feature flags work correctly for optional components."

---

## BPI-019: Fix Performance and Memory Issues
* **Summary**: Optimize event bus, AI, and engine for speed and low memory.
* **Requirements**
  * Optimize event bus performance for high-frequency events
  * Ensure bot decision making meets 60Hz performance targets
  * Optimize memory usage for long-running tournaments
  * Reduce AI component coordination overhead
* **Files that need changing**
  * `crates/events/src/bus.rs` – Optimize event bus performance
  * `crates/bot/src/ai/pipeline.rs` – Optimize AI decision making
  * `crates/engine/src/engine/game_engine.rs` – Optimize game loop
  * `crates/engine/src/tournament.rs` – Optimize tournament memory usage
  * `crates/common/src/diagnostics.rs` – Add performance monitoring
* **What needs to change**
  * Event bus must use efficient data structures for high-frequency event processing
  * Bot decisions must be optimized to complete within 16ms (60Hz) time budget
  * Tournament must manage memory efficiently for hundreds of games
  * AI components must minimize data copying and allocation overhead
  * Performance monitoring must track critical metrics and alert on degradation
* **Prompt**: "Fix performance and memory issues by optimizing event bus performance for high-frequency events, ensuring bot decision making meets 60Hz requirements, optimizing memory usage for long-running tournaments, and reducing AI component coordination overhead. Add comprehensive performance monitoring."

---

## BPI-020: Add Missing Documentation and Examples
* **Summary**: Improve user guidance with thorough API docs and examples.
* **Requirements**
  * Complete API documentation for all new components
  * Add usage examples for the integrated system
  * Update architecture documentation to reflect current state
  * Create setup and deployment guides
* **Files that need changing**
  * `Docs/` – Update all architecture documentation
  * `crates/*/src/lib.rs` – Complete API documentation
  * `examples/` – Create usage examples
  * `README.md` – Update setup instructions
  * `Docs/deployment/` – Create deployment guides
* **What needs to change**
  * All public APIs must have comprehensive documentation with examples
  * Architecture docs must reflect the current integrated system state
  * Usage examples must demonstrate event bus, bot management, and tournament functionality
  * Setup guides must include configuration options and deployment modes
  * Deployment guides must cover different environments (development, staging, production)
* **Prompt**: "Add missing documentation and examples by completing API documentation for all new components, adding usage examples for the integrated system, updating architecture documentation to reflect current implementation, and creating comprehensive setup and deployment guides."

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
