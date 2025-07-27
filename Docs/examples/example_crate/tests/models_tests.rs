use example_crate::models::Greeting;

#[test]
fn greeting_new_sets_message() {
    let g = Greeting::new("test");
    assert_eq!(g.message, "test");
}

#[test]
fn static_provider_default() {
    use example_crate::providers::{NameProvider, StaticNameProvider};
    let p = StaticNameProvider::default();
    assert_eq!(p.name().unwrap(), "World");
}
