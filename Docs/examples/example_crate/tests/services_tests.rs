use example_crate::providers::{DefaultUuidProvider, UuidProvider};
use example_crate::services::{DefaultGreetingService, GreetingService};

#[test]
fn default_greeting_service_returns_message() {
    let svc = DefaultGreetingService;
    assert_eq!(svc.greeting("Bob").unwrap(), "Hello, Bob!");
}

#[test]
fn default_uuid_provider_returns_uuid() {
    let provider = DefaultUuidProvider;
    let id = provider.uuid().unwrap();
    assert_eq!(id.get_version_num(), 4);
}
