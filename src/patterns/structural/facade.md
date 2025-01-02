### Facade Design Pattern

The Facade pattern provides a simplified interface to a complex subsystem. It defines a higher-level interface that makes the subsystem easier to use.

Here is an example of the Facade Design Pattern:

```rust
struct SubsystemA;

impl SubsystemA {
    fn operation_a1(&self) -> String {
        "Subsystem A, Operation A1".to_string()
    }

    fn operation_a2(&self) -> String {
        "Subsystem A, Operation A2".to_string()
    }
}

struct SubsystemB;

impl SubsystemB {
    fn operation_b1(&self) -> String {
        "Subsystem B, Operation B1".to_string()
    }

    fn operation_b2(&self) -> String {
        "Subsystem B, Operation B2".to_string()
    }
}

struct Facade {
    subsystem_a: SubsystemA,
    subsystem_b: SubsystemB,
}

impl Facade {
    fn new() -> Self {
        Facade {
            subsystem_a: SubsystemA,
            subsystem_b: SubsystemB,
        }
    }

    fn operation(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.subsystem_a.operation_a1());
        result.push_str("\n");
        result.push_str(&self.subsystem_a.operation_a2());
        result.push_str("\n");
        result.push_str(&self.subsystem_b.operation_b1());
        result.push_str("\n");
        result.push_str(&self.subsystem_b.operation_b2());
        result
    }
}

fn main() {
    let facade = Facade::new();
    println!("{}", facade.operation());
}
```

1. **SubsystemA and SubsystemB**: Represent the complex subsystems with their own operations.
2. **Facade Struct**: Provides a simplified interface to interact with the subsystems.
3. **new() Method**: Initializes the subsystems.
4. **operation() Method**: Provides a higher-level interface to perform operations involving multiple subsystems.
5. **main() Function**: Demonstrates the usage of the facade to interact with the subsystems.

This example shows how the Facade pattern simplifies the interaction with complex subsystems by providing a unified interface.