// Approach 1: Traditional Decorator with trait objects
// Works but requires heap allocations and dynamic dispatch

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

// Approach 2: Idiomatic Rust using Newtype pattern
// Zero-cost abstraction with compile-time guarantees

struct BaseCoffee {
    cost: f64,
    description: String,
}

impl BaseCoffee {
    fn simple() -> Self {
        BaseCoffee {
            cost: 5.0,
            description: "Simple coffee".to_string(),
        }
    }

    fn with_milk(mut self) -> Self {
        self.cost += 1.5;
        self.description.push_str(", with milk");
        self
    }

    fn with_sugar(mut self) -> Self {
        self.cost += 0.5;
        self.description.push_str(", with sugar");
        self
    }

    fn cost(&self) -> f64 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }
}

fn main() {
    println!("=== Traditional Decorator (heap allocated) ===");
    let simple_coffee = Box::new(SimpleCoffee);
    println!("Cost: ${}, Description: {}", 
        simple_coffee.cost(), 
        simple_coffee.description()
    );

    let milk_coffee = Box::new(MilkDecorator::new(simple_coffee));
    println!("Cost: ${}, Description: {}", 
        milk_coffee.cost(), 
        milk_coffee.description()
    );

    let sugar_milk_coffee = Box::new(SugarDecorator::new(milk_coffee));
    println!("Cost: ${}, Description: {}", 
        sugar_milk_coffee.cost(), 
        sugar_milk_coffee.description()
    );

    println!("\n=== Idiomatic Rust (builder pattern, zero-cost) ===");
    let coffee = BaseCoffee::simple();
    println!("Cost: ${}, Description: {}", 
        coffee.cost(), 
        coffee.description()
    );

    let coffee = coffee.with_milk();
    println!("Cost: ${}, Description: {}", 
        coffee.cost(), 
        coffee.description()
    );

    let coffee = coffee.with_sugar();
    println!("Cost: ${}, Description: {}", 
        coffee.cost(), 
        coffee.description()
    );

    // Can also chain methods
    let fancy_coffee = BaseCoffee::simple()
        .with_milk()
        .with_sugar();
    println!("\nChained: Cost: ${}, Description: {}",
        fancy_coffee.cost(),
        fancy_coffee.description()
    );
}