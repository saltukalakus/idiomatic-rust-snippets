### Decorator Design Pattern

The Decorator pattern allows behavior to be added to an individual object, dynamically, without affecting the behavior of other objects from the same class. This is useful for adhering to the Single Responsibility Principle by allowing functionality to be divided between classes with unique areas of concern.

Here is an example of the Decorator Design Pattern in Rust:

```rust
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
```

1. **Coffee Trait**: Defines the interface for the coffee with methods `cost` and `description`.
2. **SimpleCoffee Struct**: Implements the `Coffee` trait with basic cost and description.
3. **MilkDecorator Struct**: A decorator that adds milk to the coffee, increasing the cost and updating the description.
4. **SugarDecorator Struct**: A decorator that adds sugar to the coffee, increasing the cost and updating the description.
5. **main() Function**: Demonstrates the usage of the decorators to dynamically add behavior to the coffee object.

This example shows how the Decorator pattern can be used to extend the functionality of objects in a flexible and reusable way.