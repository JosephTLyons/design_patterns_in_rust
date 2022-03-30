pub trait DesignPatternExample {
    fn name<'a>(&self) -> &'a str;
    fn run(&self);
}
