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
        println!("Running: {}", example.name());
        example.run();
        println!();
    }
}
