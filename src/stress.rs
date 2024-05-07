pub trait StressStrategy {
    fn run(&mut self);
    fn name<'a>(&self) -> &'a str;
}
