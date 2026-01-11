### Behavioral Patterns

Behavioral Patterns deal with object collaboration and the delegation of responsibilities. In Rust, ownership and borrowing rules significantly influence how these patterns are implemented.

### Rust-Adapted Behavioral Patterns

1- [Message Passing](./message-passing.md): Uses channels for pub-sub communication. This approach avoids ownership conflicts and is thread-safe by default.

2- [Strategy](./strategy.md): Defines a family of algorithms, encapsulates each one, and makes them interchangeable. Works well in Rust using traits.

3- [Chain of Responsibility](./chain-of-responsibility.md): Allows an object to pass a request along a chain of potential handlers until the request is handled.

4- [Mediator](./mediator.md): Defines an object that encapsulates how a set of objects interact.

5- [Enum Polymorphism](./enum-polymorphism.md): Uses enums with pattern matching to perform different operations on different types. Provides exhaustive checking and excellent performance.
