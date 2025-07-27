#![allow(missing_docs)]
//! Dependency injection container using `shaku`.
//!
//! The [`AppModule`] struct provides access to the registered components.
//!
//! ```
//! use example_crate::container::AppModule;
//! let _module = AppModule::builder().build();
//! ```

use crate::providers::StaticNameProvider;
use crate::services::EnglishGreeter;
use shaku::module;

mod imp {
    use super::*;
    module! {
        pub AppModule {
            components = [EnglishGreeter, StaticNameProvider],
            providers = []
        }
    }
}

pub use imp::AppModule;
