#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

//! Minimal service oriented crate using `shaku` for dependency injection.
//! Optional logging is available via the `logging` feature.
//!
//! ```
//! use example_crate::{container::AppModule, processors::HelloWorldProcessor};
//! use shaku::HasComponent;
//! let module = AppModule::builder().build();
//! let greeter = module.resolve();
//! let name_provider = module.resolve();
//! let processor = HelloWorldProcessor::new(greeter, name_provider);
//! let greeting = processor.run().unwrap();
//! assert_eq!(greeting.message, "Hello, World!");
//! ```

pub mod config;
pub mod container;
pub mod error;
pub mod helpers;
pub mod models;
pub mod processors;
pub mod providers;
pub mod services;
