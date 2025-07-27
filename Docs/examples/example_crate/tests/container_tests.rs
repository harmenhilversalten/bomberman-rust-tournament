use example_crate::container::AppModule;
use example_crate::services::Greeter;
use example_crate::providers::NameProvider;
use shaku::HasComponent;

#[test]
fn resolves_components() {
    let module = AppModule::builder().build();
    let greeter: std::sync::Arc<dyn Greeter> = module.resolve();
    let name_provider: std::sync::Arc<dyn NameProvider> = module.resolve();

    let greeting = greeter.greet("Rust").unwrap();
    assert_eq!(greeting, "Hello, Rust!");
    assert_eq!(name_provider.name().unwrap(), "World");
}
