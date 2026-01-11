### Ownership and Memory Anti-Patterns

Rust's ownership system is one of its defining features, ensuring memory safety without garbage collection. However, developers sometimes fight the ownership system or misuse memory management patterns, leading to inefficient or overly complex code.

This section covers common anti-patterns related to ownership, borrowing, lifetimes, and memory management:

1- [Excessive Cloning](./excessive-cloning.md): Unnecessary use of `.clone()` when borrowing or moving would be more efficient.

2- [Unnecessary Smart Pointers](./unnecessary-smart-pointers.md): Over-using `Arc`, `Rc`, or `Box` when simple references or owned values would work.

3- [Excessive Mutability](./excessive-mutability.md): Making everything mutable when immutability would be clearer and safer.

4- [Unnecessary Unsafe](./unnecessary-unsafe.md): Using `unsafe` blocks without proper justification when safe alternatives exist.

5- [Improper Lifetime Handling](./lifetime-issues.md): Fighting lifetime errors with workarounds like excessive cloning instead of proper lifetime annotations.
