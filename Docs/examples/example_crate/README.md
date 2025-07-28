# example_crate

A tiny service oriented crate demonstrating compile-time dependency injection
with [`shaku`](https://crates.io/crates/shaku). It exposes a greeting workflow
and includes a configuration loader. Optional features enable tracing based
logging and `serde` serialization for the models.

## Quick start

```bash
# build and run tests
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
# run the demo binary with logging enabled
GREETING_PREFIX=Hi \
  cargo run --bin demo \
  --manifest-path Docs/examples/example_crate/Cargo.toml \
  --features logging
```

## Features

 - `logging` – enables tracing output and the demo binary.
- `serde` – adds serialization support for models.

## Configuration

`Config::load` reads the optional `GREETING_PREFIX` environment variable. If the
variable is unset, the prefix defaults to `"Hello"`.

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
Unit and property-based tests live under `tests/`.

## Adding a new service

1. Define a trait and implementation under `src/services` or `src/providers`.
2. Expose them from the corresponding `mod.rs`.
3. Register the component in `src/container.rs` if it should be resolved automatically.
4. Add unit tests for the new code.
