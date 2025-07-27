use example_crate::{container::AppModule, processors::HelloWorldProcessor};
use shaku::HasComponent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let module = AppModule::builder().build();
    let greeter = module.resolve();
    let name_provider = module.resolve();
    let processor = HelloWorldProcessor::new(greeter, name_provider);
    println!("{}", processor.run()?);
    Ok(())
}
