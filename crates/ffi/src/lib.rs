//! Temporary skeleton crate
#![warn(missing_docs, clippy::all)]
/// Initializes the crate and returns a greeting.
pub fn init() -> &'static str {
    "initialized"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_returns_initialized() {
        assert_eq!(init(), "initialized");
    }
}
