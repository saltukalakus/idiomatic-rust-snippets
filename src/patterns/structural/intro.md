### Structural Patterns

Structural Patterns deal with object composition and relationships between entities. Rust's ownership system and trait-based polymorphism influence how these patterns are implemented.

**Note**: Some patterns have been renamed to emphasize Rust-idiomatic approaches over traditional OOP implementations.

### Rust-Adapted Structural Patterns

1- [Adapter](./adapter.md): Allows objects with incompatible interfaces to work together. In Rust, this often uses trait implementations and newtype patterns.

2- [Bridge](./bridge.md): Decouples an abstraction from its implementation so that the two can vary independently.

3- [Composite](./composite.md): Composes objects into tree structures. In Rust, enums are often more idiomatic than trait objects for tree-like structures.

4- [Builder Pattern](./builder-pattern.md): **Rust's Idiomatic Alternative to Decorator** - Uses method chaining for zero-cost composition. Traditional decorator with trait objects works but adds heap allocation overhead.

5- [Facade](./facade.md): Provides a simplified interface to a complex subsystem, making it easier to use.

6- [Flyweight](./flyweight.md): Reduces the cost of creating and manipulating similar objects by sharing data. Rust's `Arc` and `Cow` types facilitate this.

7- [Proxy](./proxy.md): Provides a surrogate or placeholder for another object to control access to it.