use example_crate::{config::Config, container::AppModule, processors::HelloWorldProcessor};
use example_crate::error::Result;
use shaku::HasComponent;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    #[cfg(feature = "logging")]
    {
        let subscriber = FmtSubscriber::new();
        tracing::subscriber::set_global_default(subscriber).expect("set subscriber");
    }

    let cfg = Config::load()?;
    let module = AppModule::builder().build();
    let greeter = module.resolve();
    let name_provider = module.resolve();
    let processor = HelloWorldProcessor::new(greeter, name_provider);
    let greeting = processor.run()?;
    info!("{} {}", cfg.prefix, greeting.message);
    Ok(())
}
