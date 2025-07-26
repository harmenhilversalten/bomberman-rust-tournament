//! Adapter module.
//! Adapters converting between services or providers.

mod uuid_string_adapter;

pub use uuid_string_adapter::{DefaultUuidStringAdapter, UuidStringAdapter};
