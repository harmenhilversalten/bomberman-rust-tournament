use example_crate::helpers::shout;
use proptest::prelude::*;

#[test]
fn shout_turns_uppercase() {
    assert_eq!(shout("Bob"), "BOB");
}

proptest! {
    #[test]
    fn shout_matches_uppercase(s in ".*") {
        prop_assert_eq!(shout(&s), s.to_uppercase());
    }
}
