use crate::{design_pattern_example::DesignPatternExample, design_pattern_type::DesignPatternType};

struct EngineOff;

impl EngineOff {
    fn new() -> Self {
        println!("Turning Engine Off");
        Self {}
    }

    fn engine_on(self) -> Idle {
        Idle::new()
    }
}

struct Idle;

impl Idle {
    fn new() -> Self {
        println!("Idling");
        Self {}
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
        Self {}
    }
}
impl Drive for Forward {}

struct Backward;

impl Backward {
    fn new() -> Self {
        println!("Moving Backward");
        Self {}
    }
}

impl Drive for Backward {}

pub struct StatePatternExample;

impl DesignPatternExample for StatePatternExample {
    fn design_pattern_type<'a>(&self) -> DesignPatternType {
        DesignPatternType::State("Car")
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
