use example_crate::implementations::EnglishGreeter;
use example_crate::services::GreetingService;

#[test]
fn greeting_service_returns_expected_greeting() {
    let service = GreetingService::new(EnglishGreeter);
    assert_eq!(service.send_greeting("Alice"), "Hello, Alice!");
}
