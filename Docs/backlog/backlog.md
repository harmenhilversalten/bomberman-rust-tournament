# Bomberman AI Development Backlog

This backlog lists the high level features required to evolve the current project into the architecture defined in [Docs/Architecture.md](../Architecture.md). The full architecture is decomposed under `Docs/architecture/`. Each item includes a summary, key requirements and a short prompt describing the tasks to implement.  Features are ordered roughly by dependency so later tasks can rely on the preceding work.
Completed backlog items 1-29 are archived in [completed.md](completed.md).

## 30. Bot Strategy Selection
- **Summary**: Expose a configuration mechanism that lets the game select among three bot strategies: the existing legacy logic, the new goal-based bot, and the RL bot.
- **Requirements**:
  - Implement a `BotStrategy` trait and load the desired strategy at runtime, following the Strategy pattern.
  - Ensure each strategy can subscribe to engine events and operate on snapshots.
  - Provide tests that validate switching strategies via configuration.
- **Prompt**: "Add selectable bot strategies (legacy, goal-based, RL) and cover them with tests."

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
