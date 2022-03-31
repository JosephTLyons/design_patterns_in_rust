mod design_pattern_example;
mod design_patterns;

use design_pattern_example::DesignPatternExample;
use design_patterns::{
    state_pattern::StatePatternExample, template_method_pattern::TemplateMethodPatternExample,
};

fn main() {
    let examples: Vec<Box<dyn DesignPatternExample>> = vec![
        Box::new(StatePatternExample),
        Box::new(TemplateMethodPatternExample),
    ];

    for example in examples {
        let example_name = example.name();
        println!("{example_name}");

        let divider = "=".repeat(example_name.len());
        println!("{divider}");

        example.run();
        println!();
    }
}
