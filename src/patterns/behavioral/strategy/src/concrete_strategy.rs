use crate::strategy::Strategy;

pub struct ConcreteStrategyA;

impl Strategy for ConcreteStrategyA {
    fn execute(&self, data: &str) {
        println!("ConcreteStrategyA: {}", data);
    }
}

pub struct ConcreteStrategyB;

impl Strategy for ConcreteStrategyB {
    fn execute(&self, data: &str) {
        println!("ConcreteStrategyB: {}", data);
    }
}