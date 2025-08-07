# Completed Features

This document tracks implemented features and their requirements. Update it whenever a backlog item is finished or when features are changed.

## Current State

At the time of writing, none of the features outlined in [Docs/backlog/backlog.md](../backlog/backlog.md) are fully implemented. The project currently consists of a single binary crate with the following capabilities:

- Basic Bomberman game engine (`Game`) that simulates turns and handles bombs and player actions.
- Simple bots (`easy_bot` and `random_bot`) implementing a `Bot` trait for decision making.
- Multi-threaded tournament runner for running many games in parallel.

These initial components come from the original repository and serve as a foundation for the future workspace refactor.

As new backlog features are completed, list them below with a reference to the backlog section.


## Implemented Features (append-only)

- Basic Bomberman game engine (`Game`) that simulates turns and handles bombs and player actions.
- Simple bots (`easy_bot` and `random_bot`) implementing a `Bot` trait for decision making.
- Multi-threaded tournament runner for running many games in parallel.
- Workspace restructure with skeleton crates ([Backlog #1](../backlog/completed.md#1-restructure-into-workspace)).
- Core state crate structures ([Backlog #2](../backlog/completed.md#2-state-crate-%E2%80%93-core-structures)).
- Snapshot layer with immutable views ([Backlog #3](../backlog/completed.md#3-state-crate-%E2%80%93-snapshot-layer)).
- State serialization supporting binary and JSON formats ([Backlog #4](../backlog/completed.md#4-state-crate-%E2%80%93-serialization)).
- Engine core loop with delta broadcasting ([Backlog #5](../backlog/completed.md#5-engine-crate-%E2%80%93-core-loop)).
- Engine scheduler supporting parallel task execution ([Backlog #6](../backlog/completed.md#6-engine-crate-%E2%80%93-scheduler)).
- Engine systems for movement, bombs, explosions, powerups and players ([Backlog #7](../backlog/completed.md#7-engine-crate-%E2%80%93-system-modules)).
- Replay recording and determinism checks ([Backlog #8](../backlog/completed.md#8-engine-crate-%E2%80%93-replay-and-determinism)).
- Engine configuration and game rules ([Backlog #9](../backlog/completed.md#9-engine-crate-%E2%80%93-configuration)).
- Event types and bus with subscriber registration ([Backlog #10](../backlog/completed.md#10-events-crate-%E2%80%93-event-types-and-bus)).
- Event queue with priority levels and subscription filters ([Backlog #11](../backlog/completed.md#11-events-crate-%E2%80%93-queue-and-filtering)).
- Event serialization and RL transition recording ([Backlog #12](../backlog/completed.md#12-events-crate-%E2%80%93-serialization-and-recording)).
- Influence map core with danger and opportunity layers ([Backlog #13](../backlog/completed.md#13-influence-map-crate-%E2%80%93-core-map)).
- Influence map update strategies with incremental and full options ([Backlog #14](../backlog/completed.md#14-influence-map-crate-%E2%80%93-update-strategies)).
- Influence map visualization helpers and benchmarking ([Backlog #15](../backlog/completed.md#15-influence-map-crate-%E2%80%93-visualization-and-benchmarking)).
- Pathfinding algorithms A*, D* Lite and Jump Point Search ([Backlog #16](../backlog/completed.md#16-path-crate-%E2%80%93-algorithm-implementations)).
- Path grid and heuristic modules with Manhattan and Euclidean functions ([Backlog #17](../backlog/completed.md#17-path-crate-%E2%80%93-grid-and-heuristics)).
- Path cache with eviction policies and path optimization algorithms ([Backlog #18](../backlog/completed.md#18-path-crate-%E2%80%93-caching-and-optimization)).
- Bomb logic with chain reactions and explosion calculation ([Backlog #19](../backlog/completed.md#19-bombs-crate-%E2%80%93-bomb-logic)).
- Bomb placement strategies with safe and strategic options plus timing and remote detonation support ([Backlog #20](../backlog/completed.md#20-bombs-crate-%E2%80%93-placement-and-timing)).
- Bomb power effects, kicking mechanics, and analysis utilities ([Backlog #21](../backlog/completed.md#21-bombs-crate-%E2%80%93-power-and-analysis)).
- Goal definitions and planner ([Backlog #22](../backlog/completed.md#22-goals-crate-%E2%80%93-goal-definitions-and-planner)).
- Goal execution and hierarchy management ([Backlog #23](../backlog/completed.md#23-goals-crate-%E2%80%93-execution-and-hierarchy)).
- Bot kernel coordinating decision loop via channels ([Backlog #24](../backlog/completed.md#24-bot-crate-%E2%80%93-core-kernel)).
- AI modules with heuristic, reactive and planning strategies plus runtime switching ([Backlog #25](../backlog/completed.md#25-bot-crate-%E2%80%93-ai-modules)).
- Perception and action modules with memory and executor ([Backlog #26](../backlog/completed.md#26-bot-crate-%E2%80%93-perception-and-action)).
- RL policy and value estimation traits with Torch and random implementations ([Backlog #27](../backlog/completed.md#27-rl-crate-%E2%80%93-policy-and-value-estimation)).
- RL environment and training utilities with replay buffers ([Backlog #28](../backlog/completed.md#28-rl-crate-%E2%80%93-environment-and-training)).
- Engine integration of feature crates with event-driven flow ([Backlog #29](../backlog/completed.md#29-engine-integration-of-feature-crates)).
- Bot decision loop connected to engine via event bus ([Backlog #30](../backlog/completed.md#30-connect-bot-decision-loop-to-engine)).
- AI components integrated with bot kernel for goal-driven decisions ([Backlog #31](../backlog/completed.md#31-integrate-ai-components-goals-path-influence-with-bot-kernel)).
- Reinforcement learning integration enabling bots to load policies and compute rewards ([Backlog #32](../backlog/completed.md#32-implement-reinforcement-learning-integration)).
- Bomb system integrated across engine and bot with event broadcasting ([Backlog #33](../backlog/completed.md#33-implement-bomb-system-integration)).
- System crate dependencies declared with feature flags and workspace alignment ([Backlog #34](../backlog/completed.md#34-add-missing-system-dependencies)).
- Bot kernel integration with AI pipeline and lifecycle handle ([Backlog BPI-011](../backlog/completed.md#bpi-011-fix-incomplete-bot-kernel-integration)).
