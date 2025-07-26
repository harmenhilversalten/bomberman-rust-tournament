use example_crate::{config::Config, container::Container};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = Container::new(Config::new("demo"));
    let processor = container.greeter_processor();
    println!("{}", processor.process("Alice")?);
    Ok(())
}
