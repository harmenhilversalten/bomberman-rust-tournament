use example_crate::implementations::EnglishGreeter;
use example_crate::services::GreetingService;
use proptest::prelude::*;

#[test]
fn greeting_service_returns_expected_greeting() {
    let service = GreetingService::new(EnglishGreeter);
    assert_eq!(service.send_greeting("Alice"), "Hello, Alice!");
}

proptest! {
    #[test]
    fn greeting_service_prop(name in "[A-Za-z]{1,16}") {
        let service = GreetingService::new(EnglishGreeter);
        let expected = format!("Hello, {name}!");
        prop_assert_eq!(service.send_greeting(&name), expected);
    }
}
