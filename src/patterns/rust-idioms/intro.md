### Rust Idioms

These patterns are unique to Rust or have Rust-specific implementations that leverage the language's ownership system, type system, and zero-cost abstractions. They are commonly used in idiomatic Rust code and may not have direct equivalents in other languages.

**Key Characteristics**:
- Leverage Rust's ownership and borrowing system
- Provide compile-time guarantees
- Often have zero runtime cost
- Take advantage of Rust's powerful type system

### Rust-Specific Patterns

1- [Newtype Pattern](./newtype.md): Wrapping types for type safety and abstraction without runtime overhead.

2- [RAII (Resource Acquisition Is Initialization)](./raii.md): Automatic resource management using Rust's ownership and Drop trait.

3- [Extension Traits](./extension-traits.md): Adding methods to types you don't own through trait implementations.

4- [Type State Pattern](./typestate.md): Encoding state machines in the type system for compile-time state verification.
