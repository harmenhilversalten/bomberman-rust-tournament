use example_crate::helpers::normalize_name;

#[test]
fn normalize_name_trims_and_lowercases() {
    assert_eq!(normalize_name(" Bob \n"), "bob");
}
