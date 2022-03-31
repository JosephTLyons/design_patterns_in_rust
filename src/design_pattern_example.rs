pub trait DesignPatternExample {
    fn pattern_name<'a>(&self) -> &'a str;
    fn example_name<'a>(&self) -> &'a str;
    fn run(&self);
}
