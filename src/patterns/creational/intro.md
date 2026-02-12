### Creational Patterns

Creational Patterns deal with object creation mechanisms, providing flexible and reusable ways to construct objects while managing ownership and initialization.

### Rust-Adapted Creational Patterns

1- [Lazy Initialization](./lazy-initialization.md): Safe global state management using `OnceLock` and `LazyLock` for values that are initialized exactly once on first access.

2- [Factory Method](./factory-method.md): Defines an interface for creating an object, but lets subclasses alter the type of objects that will be created.

3- [Builder](./builder.md): Separates the construction of a complex object from its representation. This pattern is very common and idiomatic in Rust.
