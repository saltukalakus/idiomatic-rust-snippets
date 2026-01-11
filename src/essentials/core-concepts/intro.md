### Core Concepts

Rust has several core concepts that are fundamental to understanding and effectively using the language. These concepts are designed to ensure memory safety, concurrency, and performance without sacrificing ease of use.

1- [Ownership](./ownership.md): A set of rules that governs how a Rust program manages memory, ensuring memory safety without needing a garbage collector.

2- [Borrowing](./borrow.md): Allows you to reference a value without taking ownership of it.

3- [Lifetimes](./lifetime.md): A way of ensuring that references are valid as long as they are used, preventing dangling references and ensuring memory safety.

4- [Flow Controls](./flow-control.md): Control flow mechanisms like loops and conditional statements for program execution.

5- [Pattern Matching](./pattern-matching.md): A powerful feature that allows you to match complex data structures and execute code based on their shape.

6- [Concurrency](./concurrency.md): Powerful concurrency primitives like threads and channels, while ensuring memory safety through ownership and borrowing.

7- [Error Handling](./error-handling.md): Error handling system using the `Result` and `Option` enums, encouraging explicit error handling rather than exceptions.

8- [Macros](./macro.md): A way to write code that writes other code (metaprogramming), used for code generation and reducing boilerplate.

9- [Modules](./module.md): Rust's way of organizing code by grouping related code together.

10- [Crates](./crate.md): The unit of compilation and distribution in Rust.

11- [Functions](./function.md): Fundamental building blocks for organizing and reusing code, allowing you to encapsulate logic and perform specific tasks.

12- [Closures](./closure.md): Anonymous functions that can capture values from their surrounding environment.