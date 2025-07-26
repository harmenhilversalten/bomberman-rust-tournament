use example_crate::models::User;
use uuid::Uuid;

#[test]
fn user_new_stores_fields() {
    let id = Uuid::nil();
    let user = User::new(id, "Alice");
    assert_eq!(user.id, id);
    assert_eq!(user.name, "Alice");
}
