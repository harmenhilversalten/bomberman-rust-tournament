//! Helper utilities for working with strings.
//!
//! The [`capitalize`] function returns the input string with the first
//! character converted to uppercase.
//!
//! ```
//! use example_crate::helpers::string_helpers::capitalize;
//! assert_eq!(capitalize("hello"), "Hello");
//! ```

/// Capitalize the first character of `input`.
#[must_use]
pub fn capitalize(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
