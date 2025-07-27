use example_crate::processors::HelloWorldProcessor;
use example_crate::services::EnglishGreeter;
use example_crate::providers::StaticNameProvider;
use std::sync::Arc;

#[test]
fn hello_world_processor_runs() {
    let greeter = Arc::new(EnglishGreeter);
    let provider = Arc::new(StaticNameProvider::default());
    let processor = HelloWorldProcessor::new(greeter, provider);
    let greeting = processor.run().unwrap();
    assert_eq!(greeting.message, "Hello, World!");
}
