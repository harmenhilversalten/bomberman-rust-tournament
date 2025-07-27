//! Data models used by the crate.
//!
//! ```
//! use example_crate::models::Greeting;
//! let g = Greeting::new("hi");
//! assert_eq!(g.message, "hi");
//! ```

mod greeting;

pub use greeting::Greeting;
