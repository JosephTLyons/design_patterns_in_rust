use crate::design_pattern_example::DesignPatternExample;

struct EngineOff;

impl EngineOff {
    fn new() -> Self {
        println!("Turning Engine Off");
        EngineOff {}
    }
    fn engine_on(self) -> Idle {
        Idle::new()
    }
}

struct Idle;

impl Idle {
    fn new() -> Self {
        println!("Idling");
        Idle {}
    }

    fn forward(self) -> Forward {
        Forward::new()
    }

    fn backward(self) -> Backward {
        Backward::new()
    }

    fn engine_off(self) -> EngineOff {
        EngineOff::new()
    }
}

trait Drive {
    fn stop(self) -> Idle
    where
        Self: Sized,
    {
        Idle::new()
    }
    fn left(&self) {
        println!("Turning Left");
    }
    fn right(&self) {
        println!("Turning Right");
    }
}

struct Forward;

impl Forward {
    fn new() -> Self {
        println!("Moving Forward");
        Forward {}
    }
}
impl Drive for Forward {}

struct Backward;

impl Backward {
    fn new() -> Self {
        println!("Moving Backward");
        Backward {}
    }
}

impl Drive for Backward {}

pub struct StatePatternExample;

impl DesignPatternExample for StatePatternExample {
    fn name<'a>(&self) -> &'a str {
        "State Pattern"
    }

    fn run(&self) {
        let car = EngineOff {};
        let car = car.engine_on();
        let car = car.forward();

        car.left();
        car.right();

        let car = car.stop();
        let car = car.backward();

        car.left();
        car.right();

        let car = car.stop();
        let _ = car.engine_off();
    }
}
