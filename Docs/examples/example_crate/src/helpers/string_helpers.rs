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

/// Truncate `s` to at most `max_len` characters.
///
/// ```
/// use example_crate::helpers::string_helpers::truncate;
/// assert_eq!(truncate("abcdef", 3), "abc");
/// ```
#[must_use]
pub fn truncate(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}

/// Returns `true` if `s` contains only alphanumeric characters.
///
/// ```
/// use example_crate::helpers::string_helpers::is_alphanumeric;
/// assert!(is_alphanumeric("abc123"));
/// assert!(!is_alphanumeric("hi!"));
/// ```
#[must_use]
pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}
