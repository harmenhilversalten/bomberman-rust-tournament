Bomberman Using Rust Project - BURP

= Goals

This project is intended as a small project to discover the main features and language idiom of Rust. The main idea is 
that participants write bots, improve bots and let the system run tournaments to determine the best bot.

= What is Rust

You know C++? C#? Great, Rust provides a good way to get you finally into productive mode, especially if you are working with threads or low level code.

## Documentation

Additional documentation can be found in the `Docs/` directory. A minimal
reference crate showcasing SOLID code organization resides under
`Docs/examples`. See `Docs/examples/README.md` for details.

## Development

To keep the code base consistent, run the following commands before committing
changes:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

The `example_crate` in `Docs/examples/` can be used as a template for creating
new crates. Its tests can be executed with:

```bash
cargo test --manifest-path Docs/examples/example_crate/Cargo.toml
```

