use example_crate::models::Greeting;
use example_crate::helpers::string_helpers::truncate;
use proptest::prelude::*;

proptest! {
    #[test]
    fn greeting_roundtrip(s in ".*") {
        let g = Greeting::new(&s);
        prop_assert_eq!(g.message, s);
    }
}

proptest! {
    #[test]
    fn truncate_never_exceeds_len(s in ".*", max in 0usize..20) {
        let out = truncate(&s, max);
        prop_assert!(out.chars().count() <= max);
    }
}
