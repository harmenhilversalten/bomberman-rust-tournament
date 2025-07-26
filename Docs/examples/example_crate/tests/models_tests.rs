use example_crate::models::User;

#[test]
fn user_new_sets_name() {
    let user = User::new("Ann");
    assert_eq!(user.name, "Ann");
}
