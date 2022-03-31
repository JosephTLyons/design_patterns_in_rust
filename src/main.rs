mod design_pattern_example;
mod design_pattern_type;
mod design_patterns;

use design_pattern_example::DesignPatternExample;
use design_patterns::{
    builder_pattern::BuilderPatternExample, state_pattern::StatePatternExample,
    template_method_pattern::TemplateMethodPatternExample,
};

fn main() {
    let examples: Vec<Box<dyn DesignPatternExample>> = vec![
        Box::new(BuilderPatternExample),
        Box::new(StatePatternExample),
        Box::new(TemplateMethodPatternExample),
    ];

    for example in examples {
        let header = example.design_pattern_type().to_string();
        println!("{header}");

        let divider = "=".repeat(header.len());
        println!("{divider}");

        example.run();
        println!();
    }
}
