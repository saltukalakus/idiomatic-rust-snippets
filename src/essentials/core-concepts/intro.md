### Core Concepts

Rust has several core concepts that are fundamental to understanding and effectively using the language. These concepts are designed to ensure memory safety, concurrency, and performance without sacrificing ease of use.

1- [Ownership](./ownership.md): A set of rules that governs how a Rust program manages memory, ensuring memory safety without needing a garbage collector.

2- [Borrowing](./borrow.md): Allows you to reference a value without taking ownership of it.

3- [Lifetimes](./lifetime.md): A way of ensuring that references are valid as long as they are used, preventing dangling references and ensuring memory safety.

4- [Iterators](./iterator.md): Process sequences of elements efficiently using the Iterator trait with lazy evaluation and zero-cost abstractions.

5- [Flow Controls](./flow-control.md): Control flow mechanisms like loops and conditional statements for program execution.

6- [Pattern Matching](./pattern-matching.md): A powerful feature that allows you to match complex data structures and execute code based on their shape.

7- [Concurrency](./concurrency.md): Powerful concurrency primitives like threads and channels, while ensuring memory safety through ownership and borrowing.

8- [Async/Await](./async-await.md): Asynchronous programming with async/await syntax for efficient I/O-bound operations without blocking threads.

9- [Error Handling](./error-handling.md): Error handling system using the `Result` and `Option` enums, encouraging explicit error handling rather than exceptions.

10- [Macros](./macro.md): A way to write code that writes other code (metaprogramming), used for code generation and reducing boilerplate.

11- [Modules](./module.md): Rust's way of organizing code by grouping related code together.

12- [Crates](./crate.md): The unit of compilation and distribution in Rust.

13- [Functions](./function.md): Fundamental building blocks for organizing and reusing code, allowing you to encapsulate logic and perform specific tasks.

14- [Closures](./closure.md): Anonymous functions that can capture values from their surrounding environment.