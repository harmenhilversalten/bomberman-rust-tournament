# Bomberman Using Rust Project

This repository hosts a small Bomberman inspired tournament runner written in Rust.  
It serves as a playground to explore the language while incrementally adopting the architecture described in [`Docs/architecture`](Docs/Architecture.md).

## Running

The project is organized as a Cargo workspace. The main engine crate lives in `crates/engine`.

Build and launch a tournament with the default settings:

```bash
cargo run --release -p engine
```

Execute all unit and integration tests with:

```bash
cargo test --all
```

## Documentation

Detailed design notes live in the `Docs/` directory. A fully documented example crate demonstrating SOLID organization and compile-time dependency injection is available under `Docs/examples`. See `Docs/examples/README.md` for details.

For an overview of planned work consult the [project backlog](Docs/backlog/backlog.md).  
Progress on implemented features is tracked in [Docs/completed/features.md](Docs/completed/features.md).
