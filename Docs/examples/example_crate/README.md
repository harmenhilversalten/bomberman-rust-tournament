# Example Crate

This crate demonstrates a service-oriented architecture with a simple dependency injection container.

## Folder Structure
The template uses optional folders that can be included only when needed:

```
src/
├── config.rs      # shared configuration
├── container.rs   # DI container
├── error.rs       # crate-wide errors
├── services/      # service traits and implementations
├── providers/     # provider traits and implementations
├── processors/    # logic consuming services
├── helpers/       # small utility functions (optional)
└── models/        # data models (optional)
```

Any subset of these directories may be used. For instance, a crate might only define models or services.

## Adding a New Service
1. Define a trait in `src/services` or `src/providers`.
2. Implement the trait in its own module.
3. Expose the trait and implementation in the corresponding `mod.rs`.
4. Register the implementation in `container.rs` or provide a constructor so processors can use it.
