# Example Crate

This crate demonstrates a service-oriented architecture with a simple dependency injection container.

## Folder Layout

```
example_crate/
├── Cargo.toml
├── README.md
├── rustfmt.toml
├── justfile
└── src/
    ├── lib.rs
    ├── error.rs
    ├── config.rs
    ├── container.rs
    ├── services/
    ├── providers/
    ├── processors/
    ├── helpers/
    └── models/
```

All folders are optional. A crate may contain only the pieces it needs, for example just `models` or `services`.

## Adding a New Service
1. Define a trait in `src/services` or `src/providers`.
2. Implement the trait in its own module.
3. Expose the trait and implementation in the corresponding `mod.rs`.
4. Register the implementation in `container.rs` or provide a constructor so processors can use it.
