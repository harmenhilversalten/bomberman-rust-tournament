use example_crate::processors::GreeterProcessor;
use example_crate::providers::UuidProvider;
use example_crate::services::GreetingService;
use mockall::{mock, predicate::*};
use uuid::Uuid;

mock! {
    Greeting {}
    impl GreetingService for Greeting {
        fn greeting(&self, name: &str) -> example_crate::error::Result<String>;
    }
}

mock! {
    UuidProv {}
    impl UuidProvider for UuidProv {
        fn uuid(&self) -> example_crate::error::Result<Uuid>;
    }
}

#[test]
fn processor_uses_dependencies() {
    let mut greet = MockGreeting::new();
    greet
        .expect_greeting()
        .with(eq("Bob"))
        .returning(|_| Ok("hi".into()));

    let mut uuid = MockUuidProv::new();
    uuid.expect_uuid().returning(|| Ok(Uuid::nil()));

    let processor = GreeterProcessor::new(&greet, &uuid);
    let out = processor.process("Bob").unwrap();
    assert_eq!(out, format!("hi ({})", Uuid::nil()));
}
