### Type System Anti-Patterns

Rust's type system is designed to catch errors at compile time and express intent clearly. Fighting against the type system or misusing it leads to code that's harder to understand and maintain.

This section covers common anti-patterns related to Rust's type system:

1- [String vs str Confusion](./string-vs-str.md): Using `String` when `&str` would suffice, causing unnecessary allocations and making APIs less flexible.

2- [Fighting the Type System](./fighting-type-system.md): Using excessive type conversions or unsafe code to bypass type safety instead of leveraging Rust's type system properly.

3- [Misusing Deref Coercion](./deref-abuse.md): Overusing `Deref` trait for types that aren't smart pointers, violating the principle of least surprise and creating confusing APIs.

4- [Stringly Typed Code](./stringly-typed.md): Using strings for everything instead of proper types and enums for better type safety and compile-time guarantees.
