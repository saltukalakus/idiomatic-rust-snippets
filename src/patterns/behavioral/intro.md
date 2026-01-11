### Behavioral Patterns

Behavioral Patterns deal with object collaboration and the delegation of responsibilities. In Rust, ownership and borrowing rules significantly influence how these patterns are implemented.

**Note**: Several patterns have been renamed to reflect Rust-idiomatic approaches. Traditional OOP patterns often don't work well or are unnecessary in Rust.

### Rust-Adapted Behavioral Patterns

1- [Message Passing](./message-passing.md): **Rust Alternative to Observer** - Uses channels for pub-sub communication instead of traditional observer callbacks. This approach avoids ownership conflicts and is thread-safe by default.

2- [Strategy](./strategy.md): Defines a family of algorithms, encapsulates each one, and makes them interchangeable. Works well in Rust using traits.

3- [Command](./command.md): Encapsulates a request as an object. In Rust, closures and function pointers often provide simpler alternatives.

4- [Chain of Responsibility](./chain-of-responsibility.md): Allows an object to pass a request along a chain of potential handlers until the request is handled.

5- [Mediator](./mediator.md): Defines an object that encapsulates how a set of objects interact.

6- [Enum Polymorphism](./enum-polymorphism.md): **Rust Alternative to Visitor** - Uses enums with pattern matching instead of the traditional visitor pattern. This provides exhaustive checking and better performance.
