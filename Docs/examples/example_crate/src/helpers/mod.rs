//! Helper utilities.

/// Convert a string to uppercase.
///
/// # Examples
///
/// ```
/// use example_crate::helpers::shout;
/// assert_eq!(shout("hello"), "HELLO");
/// ```
#[must_use]
pub fn shout(input: &str) -> String {
    input.to_uppercase()
}
