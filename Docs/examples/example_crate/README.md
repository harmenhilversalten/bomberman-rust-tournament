# example_crate

A self-contained demonstration of compile-time dependency injection using
[`shaku`](https://crates.io/crates/shaku). The crate exposes a small service
oriented architecture consisting of a `Greeter` service, a `NameProvider`
provider and a `HelloWorldProcessor` that combines both. Utilities live in the
`string_helpers` module and all fallible operations use an error type based on
[`thiserror`](https://crates.io/crates/thiserror).

## Layout

```
src/
├── lib.rs            # Crate root and exports
├── container.rs      # shaku module declaration
├── error.rs          # Error definitions
├── helpers/
│   └── string_helpers.rs
├── processors/
│   └── hello_world.rs
├── providers/
│   └── name_provider.rs
└── services/
    └── greeter.rs
```

Run the example with:

```bash
cargo run --example demo --manifest-path Docs/examples/example_crate/Cargo.toml
```
