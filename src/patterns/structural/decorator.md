### Decorator Design Pattern

The Decorator pattern allows behavior to be added to an individual object, dynamically, without affecting the behavior of other objects from the same class. This is useful for adhering to the Single Responsibility Principle by allowing functionality to be divided between classes with unique areas of concern.

```rust
{{#include decorator/src/main.rs}}
```

1. **Coffee Trait**: Defines the interface for the coffee with methods `cost` and `description`.
2. **SimpleCoffee Struct**: Implements the `Coffee` trait with basic cost and description.
3. **MilkDecorator Struct**: A decorator that adds milk to the coffee, increasing the cost and updating the description.
4. **SugarDecorator Struct**: A decorator that adds sugar to the coffee, increasing the cost and updating the description.