use example_crate::services::{EnglishGreeter, Greeter};
use example_crate::providers::{StaticNameProvider, NameProvider};

#[test]
fn english_greeter_greets() {
    let g = EnglishGreeter;
    let msg = g.greet("Rustacean").unwrap();
    assert_eq!(msg, "Hello, Rustacean!");
}

#[test]
fn static_name_provider_returns_world() {
    let provider = StaticNameProvider::default();
    let name = provider.name().unwrap();
    assert_eq!(name, "World");
}
