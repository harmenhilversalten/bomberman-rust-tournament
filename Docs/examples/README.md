# Example Crate

This directory contains `example_crate`, a small project demonstrating a service-oriented architecture with a lightweight dependency injection container. The crate is organized according to the SOLID guidelines and serves as reference code for agents building new crates.

## Folder Structure
The template includes optional folders that can be added only when needed:

```text
example_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── config.rs
│   ├── container.rs
│   ├── error.rs
│   ├── services/
│   ├── providers/
│   ├── processors/
│   ├── adapters/
│   ├── helpers/
│   └── models/
└── tests/
    ├── container_tests.rs
    ├── helpers_tests.rs
    ├── models_tests.rs
    ├── processor_tests.rs
    └── services_tests.rs
```

Services expose traits and baseline implementations. Providers offer external data while processors combine these pieces into higher level logic. Adapters translate between interfaces when needed. Helpers and models host utilities and data structures. This layout keeps units small and straightforward to test.

## Adding a New Service
1. Define a trait in `src/services` or `src/providers`.
2. Implement the trait in its own module.
3. Expose the trait and implementation from the corresponding `mod.rs`.
4. Register the implementation in `container.rs` or expose a constructor so processors can use it.

## Running Tests
Run the tests from the repository root with:

```bash
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
```
