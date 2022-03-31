use crate::{design_pattern_example::DesignPatternExample, design_pattern_type::DesignPatternType};

pub struct BuilderPatternExample;

impl DesignPatternExample for BuilderPatternExample {
    fn design_pattern_type<'a>(&self) -> DesignPatternType {
        DesignPatternType::Builder("?")
    }

    fn run(&self) {}
}
