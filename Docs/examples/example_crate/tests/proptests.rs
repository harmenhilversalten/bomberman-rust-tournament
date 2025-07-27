use example_crate::models::Greeting;
use proptest::prelude::*;

proptest! {
    #[test]
    fn greeting_roundtrip(s in ".*") {
        let g = Greeting::new(&s);
        prop_assert_eq!(g.message, s);
    }
}
