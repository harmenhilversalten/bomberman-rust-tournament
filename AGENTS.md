# Agent Guidelines

All development work in this repository must align with the architecture laid out in [Docs/Architecture.md](Docs/Architecture.md).  The following rules apply to every pull request:

1. **Reference Architecture** – When implementing features, consult the Architecture document and the backlog in `Docs/backlog/backlog.md` as well as the requirements in `Docs/backlog/requirements.md`. Keep modules small and respect the crate boundaries.
2. **SOLID & Clean Code** – Follow SOLID principles.  Organize code into small files and avoid monolithic modules.  Use clear naming and maintainable abstractions.
3. **Example Crate Template** – Use `Docs/examples/README.md` as the template when creating new crates. The example showcases compile-time dependency injection with [`shaku`](https://crates.io/crates/shaku) and includes unit and property-based tests. When modifying the example crate, run `cargo test --manifest-path Docs/examples/example_crate/Cargo.toml`. Do **not** change the `Docs/examples` project unless instructions explicitly request it.
4. **Tests Required** – Every new feature must come with unit tests (and integration tests when appropriate).  Property-based tests with `proptest` should be used for complex logic.
5. **Performance Focus** – Aim for ≤1 ms median decision time per bot and ≤16 MB memory usage as described in Section 3 of the architecture.
6. **Code Safety** – Each crate should include `#![forbid(unsafe_code)]` unless implementing FFI bindings.  Use safe Rust APIs.
7. **CI Checks** – Before committing, run:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```
   Additional checks like `cargo miri test` and `cargo bench` should be executed when workflows or code changes require them.
8. **Documentation** – Update README and relevant docs when behavior or APIs change.  When a backlog item is completed or functionality is removed, update `Docs/completed/features.md` so it reflects the current repository state.  Maintain the **Implemented Features** list in that file by appending new entries and never deleting existing ones.
9. **Feature Flags** – Gate optional code with Cargo feature flags so agents can easily locate functionality.

These guidelines ensure the project remains maintainable and performant while gradually implementing the full architecture.
