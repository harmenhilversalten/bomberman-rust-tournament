#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all)]

//! Minimal service oriented crate using `shaku` for dependency injection.
//!
//! ```
//! use example_crate::{container::AppModule, processors::HelloWorldProcessor};
//! use shaku::HasComponent;
//! let module = AppModule::builder().build();
//! let greeter = module.resolve();
//! let name_provider = module.resolve();
//! let processor = HelloWorldProcessor::new(greeter, name_provider);
//! assert_eq!(processor.run().unwrap(), "Hello, World!");
//! ```

pub mod container;
pub mod error;
pub mod helpers;
pub mod processors;
pub mod providers;
pub mod services;
