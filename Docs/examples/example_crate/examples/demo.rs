use example_crate::{config::Config, container::AppModule, processors::HelloWorldProcessor};
use shaku::HasComponent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::load()?;
    let module = AppModule::builder().build();
    let greeter = module.resolve();
    let name_provider = module.resolve();
    let processor = HelloWorldProcessor::new(greeter, name_provider);
    let greeting = processor.run()?;
    println!("{} {}", cfg.prefix, greeting.message);
    Ok(())
}
