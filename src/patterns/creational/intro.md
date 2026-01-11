### Creational Patterns

Creational Patterns deal with object creation mechanisms. In Rust, these patterns often look different from traditional OOP implementations due to ownership, borrowing, and Rust's type system.

**Note**: Some patterns have been renamed to reflect Rust-idiomatic approaches rather than traditional OOP patterns. Prototype has been removed as it's just the `Clone` trait in Rust.

### Rust-Adapted Creational Patterns

1- [Lazy Initialization](./lazy-initialization.md): **Rust Alternative to Singleton** - Safe global state management using `OnceLock` and `LazyLock` without unsafe code. Rust discourages traditional singletons in favor of dependency injection.

2- [Factory Method](./factory-method.md): Defines an interface for creating an object, but lets subclasses alter the type of objects that will be created.

3- [Abstract Factory](./abstract-factory.md): Provides an interface for creating families of related or dependent objects without specifying their concrete classes.

4- [Builder](./builder.md): Separates the construction of a complex object from its representation. This pattern is very common and idiomatic in Rust.
