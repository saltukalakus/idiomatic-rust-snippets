mod strategy;
mod concrete_strategy;
mod context;

use concrete_strategy::ConcreteStrategyA;
use concrete_strategy::ConcreteStrategyB;
use context::Context;

fn main() {
    let strategy_a 
    = Box::new(ConcreteStrategyA);
    let strategy_b = Box::new(ConcreteStrategyB);

    let mut context = Context::new(strategy_a);
    context.execute_strategy("Hello, World!");

    context.set_strategy(strategy_b);
    context.execute_strategy("Hello, Rust!");
}