trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        5.0
    }

    fn description(&self) -> String {
        "Simple coffee".to_string()
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 1.5
    }

    fn description(&self) -> String {
        format!("{}, with milk", self.coffee.description())
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{}, with sugar", self.coffee.description())
    }
}

fn main() {
    let simple_coffee = Box::new(SimpleCoffee);
    println!("Cost: {}, Description: {}", simple_coffee.cost(), simple_coffee.description());

    let milk_coffee = Box::new(MilkDecorator::new(simple_coffee));
    println!("Cost: {}, Description: {}", milk_coffee.cost(), milk_coffee.description());

    let sugar_milk_coffee = Box::new(SugarDecorator::new(milk_coffee));
    println!("Cost: {}, Description: {}", sugar_milk_coffee.cost(), sugar_milk_coffee.description());
}