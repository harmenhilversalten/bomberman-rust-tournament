use example_crate::{container::AppModule, processors::HelloWorldProcessor};
use proptest::prelude::*;
use shaku::HasComponent;

#[test]
fn processor_outputs_greeting() {
    let module = AppModule::builder().build();
    let greeter = module.resolve();
    let name_provider = module.resolve();
    let processor = HelloWorldProcessor::new(greeter, name_provider);
    let out = processor.run().unwrap();
    assert_eq!(out, "Hello, World!");
}

proptest! {
    #[test]
    fn capitalize_matches_standard(s in "[a-z]*") {
        use example_crate::helpers::string_helpers::capitalize;
        if s.is_empty() {
            prop_assert_eq!(capitalize(&s), "");
        } else {
            let mut chars = s.chars();
            let first = chars.next().unwrap().to_uppercase().collect::<String>();
            let expected = first + chars.as_str();
            prop_assert_eq!(capitalize(&s), expected);
        }
    }
}
