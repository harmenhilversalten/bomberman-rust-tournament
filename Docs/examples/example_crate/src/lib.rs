#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]

//! Project-agnostic service-oriented template.
//!
//! ```
//! use example_crate::{config::Config, container::Container};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let container = Container::new(Config::new("demo"));
//! let msg = container.greeter_processor().process("Bob")?;
//! println!("{msg}");
//! # Ok(()) }
//! ```

pub mod adapters;
pub mod config;
pub mod container;
pub mod error;
pub mod helpers;
pub mod models;
pub mod processors;
pub mod providers;
pub mod services;
