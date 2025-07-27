//! Provides names for the application.
//!
//! ```
//! use example_crate::providers::{NameProvider, StaticNameProvider};
//! let provider = StaticNameProvider::default();
//! assert_eq!(provider.name().unwrap(), "World");
//! ```

use crate::error::Result;
use shaku::{Component, Interface};

/// Supplies a name value.
pub trait NameProvider: Interface {
    /// Return a name.
    fn name(&self) -> Result<String>;
}

/// Static implementation returning a fixed name.
#[derive(Component)]
#[shaku(interface = NameProvider)]
pub struct StaticNameProvider {
    /// Name returned by [`name`](NameProvider::name).
    #[shaku(default = "World".to_owned())]
    name: String,
}

impl Default for StaticNameProvider {
    fn default() -> Self {
        Self { name: "World".into() }
    }
}

impl NameProvider for StaticNameProvider {
    fn name(&self) -> Result<String> {
        Ok(self.name.clone())
    }
}
