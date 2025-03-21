### Core Concepts

Rust has several core concepts that are fundamental to understanding and effectively using the language. These concepts are designed to ensure memory safety, concurrency, and performance without sacrificing ease of use.

### 1. [Ownership](./ownership.md)

Ownership is a set of rules that governs how a Rust program manages memory. It ensures memory safety without needing a garbage collector.

### 2. [Borrowing](./borrow.md) and [References](./reference.md)

Borrowing allows you to reference a value without taking ownership of it.

### 3. [Lifetimes](./lifetime.md)

Lifetimes are a way of ensuring that references are valid as long as they are used. They prevent dangling references and ensure memory safety. Lifetimes are often inferred by the compiler, but they can also be explicitly specified.

### 4. [Flow Controls](./flow-control.md) and [Pattern Matching](./pattern-matching.md)

Pattern matching is a powerful feature that allows you to match complex data structures and execute code based on their shape. It is commonly used with `match` statements and `if let` expressions.

### 5. [Traits](../types/trait.md)

Traits help defining shared behavior. They are similar to interfaces in other languages. Traits allow you to define methods that can be implemented by different types.

### 6. [Enums](../types/enum.md) and [Pattern Matching](./pattern-matching.md)

Enums in Rust are more powerful than in many other languages because each variant can have associated data. Enums are often used with pattern matching to handle different cases.

### 7. [Concurrency](./concurency.md)

Rust provides powerful concurrency primitives, such as threads and channels, while ensuring memory safety. The ownership and borrowing system helps prevent data races at compile time.

### 8. [Error Handling](./error-handling.md)

Error handling system uses the `Result` and `Option` enums. This encourages handling errors explicitly rather than relying on exceptions.

### 9. [Macros](./macro.md)

Macros provide a way to write code that writes other code (metaprogramming). They are used for code generation and to reduce boilerplate.

### 10. [Modules](./module.md) and [Crates](./crate.md)

Modules and crates are Rust's way of organizing code. Modules allow you to group related code together, while crates are the unit of compilation and distribution.

### 11. [Functions](./function.md) and [Closures](./closure.md)

Functions are a fundamental building block for organizing and reusing code. Functions allow you to encapsulate logic, perform specific tasks, and return values.