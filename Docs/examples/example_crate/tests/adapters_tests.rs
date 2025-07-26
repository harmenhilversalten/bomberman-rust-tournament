use example_crate::adapters::{DefaultUuidStringAdapter, UuidStringAdapter};
use example_crate::providers::UuidProvider;
use mockall::{mock, predicate::*};
use uuid::Uuid;

mock! {
    Provider {}
    impl UuidProvider for Provider {
        fn uuid(&self) -> example_crate::error::Result<Uuid>;
    }
}

#[test]
fn adapter_converts_uuid_to_string() {
    let mut provider = MockProvider::new();
    provider.expect_uuid().returning(|| Ok(Uuid::nil()));

    let adapter = DefaultUuidStringAdapter::new(&provider);
    let out = adapter.uuid_string().unwrap();
    assert_eq!(out, Uuid::nil().to_string());
}
