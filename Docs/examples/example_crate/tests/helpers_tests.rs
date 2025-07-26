use example_crate::helpers::shout;

#[test]
fn shout_turns_uppercase() {
    assert_eq!(shout("Bob"), "BOB");
}
