use crate::design_pattern_example::DesignPatternExample;

struct EngineOff;

impl EngineOff {
    fn engine_on(self) -> Idle {
        Idle {}
    }
}

struct Idle;

impl Idle {
    fn forward(self) -> Forward {
        Forward {}
    }

    fn backward(self) -> Backward {
        Backward {}
    }

    fn engine_off(self) -> EngineOff {
        EngineOff {}
    }
}

trait Drive {
    fn stop(self) -> Idle
    where
        Self: Sized,
    {
        Idle {}
    }
    fn left(&self) {}
    fn right(&self) {}
}

struct Forward;
impl Drive for Forward {}

struct Backward;
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
