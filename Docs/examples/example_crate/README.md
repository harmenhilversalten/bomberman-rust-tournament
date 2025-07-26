# example_crate

> A minimal, idiomatic, AI-friendly Rust crate demonstrating
> service-oriented architecture with dependency injection.


src/
├── lib.rs          # Crate root & public re-exports
├── config.rs       # Configuration structs
├── container.rs    # Lightweight DI container
├── error.rs        # Error type via `thiserror`
├── services/       # Traits + impls for business logic
├── providers/      # External data providers
├── processors/     # High-level orchestration
├── adapters/       # Interface adapters (e.g., tracing)
├── helpers/        # Small utilities
└── models/         # Plain data structures
# User-provided custom instructions

Make sure code is SOLID and is made with manageable architecture in mind.
Implement unittests for each new feature added.
