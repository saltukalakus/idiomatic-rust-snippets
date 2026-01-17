// 方法 1：使用 trait 对象的传统装饰器模式
// 有效但需要堆分配和动态分派

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
        "简单咖啡".to_string()
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
        format!("{}，加牛奶", self.coffee.description())
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
        format!("{}，加糖", self.coffee.description())
    }
}

// 方法 2：使用新类型模式的惯用 Rust 方式
// 零成本抽象，具有编译时保证

struct BaseCoffee {
    cost: f64,
    description: String,
}

impl BaseCoffee {
    fn simple() -> Self {
        BaseCoffee {
            cost: 5.0,
            description: "简单咖啡".to_string(),
        }
    }

    fn with_milk(mut self) -> Self {
        self.cost += 1.5;
        self.description.push_str("，加牛奶");
        self
    }

    fn with_sugar(mut self) -> Self {
        self.cost += 0.5;
        self.description.push_str("，加糖");
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
    println!("Cost: ${}, 描述: {}", 
        simple_coffee.cost(), 
        simple_coffee.description()
    );

    let milk_coffee = Box::new(MilkDecorator::new(simple_coffee));
    println!("Cost: ${}, 描述: {}", 
        milk_coffee.cost(), 
        milk_coffee.description()
    );

    let sugar_milk_coffee = Box::new(SugarDecorator::new(milk_coffee));
    println!("Cost: ${}, 描述: {}", 
        sugar_milk_coffee.cost(), 
        sugar_milk_coffee.description()
    );

    println!("\n=== Idiomatic Rust (构建器 pattern, zero-cost) ===");
    let coffee = BaseCoffee::simple();
    println!("Cost: ${}, 描述: {}", 
        coffee.cost(), 
        coffee.description()
    );

    let coffee = coffee.with_milk();
    println!("Cost: ${}, 描述: {}", 
        coffee.cost(), 
        coffee.description()
    );

    let coffee = coffee.with_sugar();
    println!("Cost: ${}, 描述: {}", 
        coffee.cost(), 
        coffee.description()
    );

    // Can also 链 methods
    let fancy_coffee = BaseCoffee::simple()
        .with_milk()
        .with_sugar();
    println!("\nChained: Cost: ${}, 描述: {}",
        fancy_coffee.cost(),
        fancy_coffee.description()
    );
}