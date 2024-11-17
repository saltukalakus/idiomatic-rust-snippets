use crate::strategy::Strategy;

pub struct Context {
    strategy: Box<dyn Strategy>,
}

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