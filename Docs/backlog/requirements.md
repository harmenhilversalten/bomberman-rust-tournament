# Game Requirements

This document summarizes the functional and non-functional requirements extracted from [Docs/Architecture.md](../Architecture.md). Detailed sections live under `Docs/architecture/`.

## Functional Requirements
- **Grid Based Map**: The game world is an N×N grid of `Tile` values. Tiles may be empty, indestructible walls, destructible crates or power‑ups.
- **Tick Based Simulation**: The engine advances in fixed time steps (ticks) at 60 FPS (16 ms per tick).
- **Bombs and Blast Waves**: Bombs explode after a timer, creating a Manhattan‑shaped blast wave halted by walls or agents.
- **Macro and Micro Moves**: Agents plan macro‑moves composed of atomic micro actions (`Move::North`, `Move::South`, etc.).
- **Influence Maps**: Danger levels across upcoming ticks are represented in influence maps for path and goal planning.
- **Goals and Planning**: Agents select high‑level goals (e.g., destroy a crate or obtain a power‑up) and build plans consisting of micro moves and bomb placements.
- **State Deltas**: Updates to game state are published as compact deltas for efficient incremental computation.

## Non-Functional Requirements
- **Performance**: ≤1 ms median decision time per bot with ≤2 ms 95th percentile latency. Each bot is limited to 16 MB memory.
- **Determinism**: The simulation must be deterministic given the same RNG seed, allowing exact replays.
- **Safety**: Unsafe Rust code is forbidden (except FFI) and the codebase should be Miri compatible.
- **Extensibility**: New game elements can be implemented in under 100 lines of code via trait-based interfaces and plugins.
- **Concurrency**: The engine and bots run concurrently using async tasks or threads, communicating via channels.

These requirements guide all development tasks and are reflected in the backlog features.
