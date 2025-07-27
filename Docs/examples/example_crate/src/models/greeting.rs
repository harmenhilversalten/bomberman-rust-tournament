//! Greeting data structure returned by processors.
//!
//! ```
//! use example_crate::models::Greeting;
//! let g = Greeting::new("hi");
//! assert_eq!(g.message, "hi");
//! ```

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Simple wrapper around a greeting message.
pub struct Greeting {
    /// Greeting message
    pub message: String,
}

impl Greeting {
    /// Create a new greeting from the provided `message`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}
