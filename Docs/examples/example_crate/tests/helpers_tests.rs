use example_crate::helpers::string_helpers::{capitalize, is_alphanumeric, truncate};

#[test]
fn capitalize_basic() {
    assert_eq!(capitalize("hello"), "Hello");
    assert_eq!(capitalize("").as_str(), "");
}

#[test]
fn truncate_shortens_string() {
    assert_eq!(truncate("abcdef", 4), "abcd");
    assert_eq!(truncate("abc", 10), "abc");
}

#[test]
fn alphanumeric_check() {
    assert!(is_alphanumeric("abc123"));
    assert!(!is_alphanumeric("abc!"));
}
