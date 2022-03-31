use std::fmt::{Display, Result};

pub enum DesignPatternType<'a> {
    Builder(&'a str),
    State(&'a str),
    TemplateMethod(&'a str),
}

impl<'a> Display for DesignPatternType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let (design_pattern_name, example_description) = match self {
            DesignPatternType::Builder(example_description) => ("Builder", example_description),
            DesignPatternType::State(example_description) => ("State", example_description),
            DesignPatternType::TemplateMethod(example_description) => {
                ("Template Method", example_description)
            }
        };

        write!(
            f,
            "{} Pattern ({})",
            design_pattern_name, example_description
        )
    }
}
