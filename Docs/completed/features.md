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
