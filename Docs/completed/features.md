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
- Workspace restructure with skeleton crates ([Backlog #1](../backlog/backlog.md#1-restructure-into-workspace)).
- Core state crate structures ([Backlog #2](../backlog/backlog.md#2-state-crate-%E2%80%93-core-structures)).
- Snapshot layer with immutable views ([Backlog #3](../backlog/backlog.md#3-state-crate-%E2%80%93-snapshot-layer)).
- State serialization supporting binary and JSON formats ([Backlog #4](../backlog/backlog.md#4-state-crate-%E2%80%93-serialization)).
- Engine core loop with delta broadcasting ([Backlog #5](../backlog/backlog.md#5-engine-crate-%E2%80%93-core-loop)).
- Engine scheduler supporting parallel task execution ([Backlog #6](../backlog/backlog.md#6-engine-crate-%E2%80%93-scheduler)).
- Engine systems for movement, bombs, explosions, powerups and players ([Backlog #7](../backlog/backlog.md#7-engine-crate-%E2%80%93-system-modules)).
- Replay recording and determinism checks ([Backlog #8](../backlog/backlog.md#8-engine-crate-%E2%80%93-replay-and-determinism)).
