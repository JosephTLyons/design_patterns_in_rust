use crate::design_pattern_type::DesignPatternType;

pub trait DesignPatternExample {
    fn design_pattern_type(&self) -> DesignPatternType;
    fn run(&self);
}
