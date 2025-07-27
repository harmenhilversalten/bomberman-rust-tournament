# example_crate

A tiny service oriented crate demonstrating compile-time dependency injection
with [`shaku`](https://crates.io/crates/shaku). It exposes a greeting workflow
and includes optional logging and a small configuration loader.

## Quick start

```bash
# build and run tests
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
# run the example
cargo run --example demo --manifest-path Docs/examples/example_crate/Cargo.toml
```

## Architecture

```
+-----------------------+
|   container::AppModule|
+-----------------------+
          | resolves
          v
+-----------+    +---------------+
| services  |    | providers     |
| Greeter   |    | NameProvider  |
+-----------+    +---------------+
          \           /
           \         /
            v       v
      +-----------------+
      | processors      |
      | HelloWorldProc. |
      +-----------------+
              |
              v
      +---------------+
      | models::Greeting|
      +---------------+
```

All public types include thorough documentation with runnable examples.

## Adding a new service

1. Define a trait and implementation under `src/services` or `src/providers`.
2. Expose them from the corresponding `mod.rs`.
3. Register the component in `src/container.rs` if it should be resolved automatically.
4. Add unit tests for the new code.
