### Strategy Pattern

The strategy pattern defines a family of interchangeable algorithms. Each algorithm is encapsulated in its own type, allowing them to be swapped at runtime.

**Benefits**:
- Swap algorithms at runtime
- Eliminates conditional logic for algorithm selection
- Each algorithm is independently testable
- Easy to add new strategies without modifying existing code

```rust, editable
{{#include strategy/src/main.rs}}
```

**Key Points**:
- The example defines `Strategy` trait with `execute(&self, data: &str)` method
- `ConcreteStrategyA` and `ConcreteStrategyB` implement different execution behaviors
- `Context` holds `Box<dyn Strategy>` and delegates to it via `execute_strategy()`
- In `main()`, context starts with strategy A, executes, then switches to strategy B with `set_strategy()`
- For compile-time dispatch (zero-cost), use generics: `Context<S: Strategy>` instead of `Box<dyn Strategy>`

**When to Use**:
- Multiple algorithms for the same task
- Runtime algorithm selection based on configuration
- Replacing conditional logic with polymorphism
- Sorting, compression, encryption with multiple implementations