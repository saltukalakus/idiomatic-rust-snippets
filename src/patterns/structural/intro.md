### Structural Patterns

Structural Patterns deal with object composition and relationships between entities. Rust's ownership system and trait-based polymorphism influence how these patterns are implemented.

### Rust-Adapted Structural Patterns

1- [Adapter](./adapter.md): Allows objects with incompatible interfaces to work together. In Rust, this often uses trait implementations and newtype patterns.

2- [Bridge](./bridge.md): Decouples an abstraction from its implementation so that the two can vary independently.

3- [Builder Pattern](./builder-pattern.md): Uses method chaining for zero-cost composition without heap allocation overhead.

4- [Facade](./facade.md): Provides a simplified interface to a complex subsystem, making it easier to use.

5- [Flyweight](./flyweight.md): Reduces the cost of creating and manipulating similar objects by sharing data. Rust's `Arc` and `Cow` types facilitate this.

6- [Proxy](./proxy.md): Provides a surrogate or placeholder for another object to control access to it.
