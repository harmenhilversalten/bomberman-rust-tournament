# example_crate

A minimal service oriented crate demonstrating compile-time dependency injection
with [`shaku`](https://crates.io/crates/shaku).  The crate exposes a greeting
workflow used throughout the documentation and tests.

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
