// Strategy type
pub trait Strategy {
    fn execute(&self, data: &str);
}

pub struct ConcreteStrategyA;

// Concrete Strategy A
impl Strategy for ConcreteStrategyA {
    fn execute(&self, data: &str) {
        println!("ConcreteStrategyA: {}", data);
    }
}

pub struct ConcreteStrategyB;

// Concrete Strategy B
impl Strategy for ConcreteStrategyB {
    fn execute(&self, data: &str) {
        println!("ConcreteStrategyB: {}", data);
    }
}

// Context type
pub struct Context {
    strategy: Box<dyn Strategy>,
}

// Concrete Context
impl Context {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    pub fn execute_strategy(&self, data: &str) {
        self.strategy.execute(data);
    }
}

fn main() {
    let strategy_a 
    = Box::new(ConcreteStrategyA);
    let strategy_b = Box::new(ConcreteStrategyB);

    let mut context = Context::new(strategy_a);
    context.execute_strategy("Hello, World!");

    context.set_strategy(strategy_b);
    context.execute_strategy("Hello, Rust!");
}