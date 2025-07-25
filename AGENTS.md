# Agent Guidelines

All development work in this repository must align with the architecture laid out in [Docs/Architecture.md](Docs/Architecture.md).  The following rules apply to every pull request:

1. **Reference Architecture** – When implementing features, consult the Architecture document and the backlog in `Docs/backlog/backlog.md` as well as the requirements in `Docs/backlog/requirements.md`. Keep modules small and respect the crate boundaries.
2. **SOLID & Clean Code** – Follow SOLID principles.  Organize code into small files and avoid monolithic modules.  Use clear naming and maintainable abstractions.
3. **Tests Required** – Every new feature must come with unit tests (and integration tests when appropriate).  Property-based tests with `proptest` should be used for complex logic.
4. **Performance Focus** – Aim for ≤1 ms median decision time per bot and ≤16 MB memory usage as described in Section 3 of the architecture.
5. **Code Safety** – Each crate should include `#![forbid(unsafe_code)]` unless implementing FFI bindings.  Use safe Rust APIs.
6. **CI Checks** – Before committing, run:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```
   Additional checks like `cargo miri test` and `cargo bench` should be executed when workflows or code changes require them.
7. **Documentation** – Update README and relevant docs when behavior or APIs change.  When a backlog item is completed or functionality is removed, update `Docs/completed/features.md` so it reflects the current repository state.  Maintain the **Implemented Features** list in that file by appending new entries and never deleting existing ones.
8. **Feature Flags** – Gate optional code with Cargo feature flags so agents can easily locate functionality.

These guidelines ensure the project remains maintainable and performant while gradually implementing the full architecture.
