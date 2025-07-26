# Completed Features

This document tracks implemented features and their requirements. Update it whenever a backlog item is finished or when features are changed.

## Current State

At the time of writing, none of the features outlined in [Docs/backlog/backlog.md](../backlog/backlog.md) are fully implemented. The project currently consists of a single binary crate with the following capabilities:

- Basic Bomberman game engine (`Game`) that simulates turns and handles bombs and player actions.
- Simple bots (`easy_bot` and `random_bot`) implementing a `Bot` trait for decision making.
- Multi-threaded tournament runner for running many games in parallel.

These initial components come from the original repository and serve as a foundation for the future workspace refactor.

As new backlog features are completed, list them below with a reference to the backlog section.

```
<!-- Example entry format (remove when first feature is complete):
### Workspace Restructure (Backlog #1)
Implemented crates `state` and `engine` with minimal APIs. Added unit tests for workspace build. -->
```

## Implemented Features (append-only)

- Basic Bomberman game engine (`Game`) that simulates turns and handles bombs and player actions.
- Simple bots (`easy_bot` and `random_bot`) implementing a `Bot` trait for decision making.
- Multi-threaded tournament runner for running many games in parallel.
