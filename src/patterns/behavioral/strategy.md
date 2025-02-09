### Strategy Pattern

The strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable. This pattern lets the algorithm vary independently from clients that use it.

Here's an example:

```rust
{{#include strategy/src/main.rs}}
```

Strategy Trait defines a common interface for all supported algorithms. Concrete Strategies implement the Strategy trait for different algorithms. Context maintains a reference to a Strategy object and is configured with a Concrete Strategy object.

This pattern is useful for scenarios where you need to switch between different algorithms or behaviors at runtime.