# Example Crate

This directory provides a minimal Rust crate named `example_crate` that
illustrates how to organize code following SOLID principles.
The layout keeps traits, implementations and services in separate
folders so that components remain easy to test and extend.

```
example_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── traits/
│   │   └── greeter.rs
│   ├── implementations/
│   │   └── english_greeter.rs
│   └── services/
│       └── greeting_service.rs
└── tests/
    └── greeting_service_tests.rs
```

`traits` defines abstractions (`Greeter`) that services depend on.
Concrete implementations live in the `implementations` folder. Services
compose these implementations through generic parameters, enabling loose
coupling and straightforward unit testing.

Run the tests with:

```bash
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
```

This crate is intended as reference code for agents wanting to build
manageable, well-structured crates.
