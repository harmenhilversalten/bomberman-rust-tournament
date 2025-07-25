use example_crate::implementations::EnglishGreeter;
use example_crate::helpers::EmojiFormatter;
use example_crate::services::GreetingService;
use proptest::prelude::*;

#[test]
fn greeting_service_returns_expected_greeting() {
    let service = GreetingService::new(EnglishGreeter);
    assert_eq!(service.send_greeting("Alice"), "Hello, Alice!");
}

#[test]
fn greeting_service_formats_output_with_emoji() {
    let service = GreetingService::with_formatter(
        EnglishGreeter,
        Box::new(EmojiFormatter),
    );
    assert_eq!(service.send_greeting("Alice"), "Hello, Alice! \u{1F60A}");
}

proptest! {
    #[test]
    fn greeting_service_prop(name in "[A-Za-z]{1,16}") {
        let service = GreetingService::new(EnglishGreeter);
        let expected = format!("Hello, {name}!");
        prop_assert_eq!(service.send_greeting(&name), expected);
    }

    #[test]
    fn greeting_service_with_formatter_prop(name in "[A-Za-z]{1,16}") {
        let service = GreetingService::with_formatter(
            EnglishGreeter,
            Box::new(EmojiFormatter),
        );
        let expected = format!("Hello, {name}! \u{1F60A}");
        prop_assert_eq!(service.send_greeting(&name), expected);
    }
}
