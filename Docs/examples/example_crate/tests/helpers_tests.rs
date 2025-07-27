use example_crate::helpers::string_helpers::capitalize;

#[test]
fn capitalize_basic() {
    assert_eq!(capitalize("hello"), "Hello");
    assert_eq!(capitalize("").as_str(), "");
}
