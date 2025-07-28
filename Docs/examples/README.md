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
│   ├── bin/
│   │   └── demo.rs
│   ├── services/
│   ├── providers/
│   ├── processors/
│   ├── helpers/
│   └── models/
└── tests/
    ├── config_tests.rs
    ├── container_tests.rs
    ├── helpers_tests.rs
    ├── integration_test.rs
    ├── models_tests.rs
    ├── processor_tests.rs
    ├── proptests.rs
    └── services_tests.rs
```

Services expose traits and baseline implementations. Providers offer external data while processors combine these pieces into higher level logic. Helpers and models host utilities and data structures. This layout keeps units small and straightforward to test.

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
