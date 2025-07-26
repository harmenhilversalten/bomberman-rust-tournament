# Example Crate

This directory contains `example_crate`, a small project showcasing
how to structure Rust code according to the SOLID guidelines.  The
crate uses a lightweight dependency injection container and keeps
services, providers and processors in dedicated folders for easy
testing and extensibility.

```
example_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── config.rs
│   ├── container.rs
│   ├── error.rs
│   ├── helpers/
│   ├── models/
│   ├── processors/
│   ├── providers/
│   └── services/
└── tests/
    ├── container_tests.rs
    ├── helpers_tests.rs
    ├── models_tests.rs
    ├── processor_tests.rs
    └── services_tests.rs
```

`services` expose traits and basic implementations. `providers` offer
external data, while `processors` combine these pieces into higher level
logic. Models and helper functions can be added as needed. This layout
keeps units small and straightforward to test.

Run the tests with:

```bash
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
```

This crate is intended as reference code for agents wanting to build
manageable, well-structured crates.
