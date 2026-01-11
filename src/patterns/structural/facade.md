### Facade Pattern

The facade pattern provides a simplified, high-level interface to a complex subsystem. It makes the subsystem easier to use by hiding internal complexity.

**Benefits**:
- Simplifies complex subsystems with a clean API
- Reduces coupling between clients and subsystem components
- Provides a single entry point for common operations
- Makes subsystems easier to test and refactor

```rust
{{#include facade/src/main.rs}}
```

**Key Points**:
- The example shows `SubsystemA` and `SubsystemB` with their own `operation()` methods
- `Facade` struct wraps both subsystems as private fields
- `Facade::operation()` provides a single entry point that coordinates both subsystems
- In `main()`, client calls `facade.operation()` instead of managing subsystems directly
- Facade simplifies the interface - client doesn't need to know about subsystem details or order of operations

**When to Use**:
- Complex subsystems with many interdependent classes
- Simplify library or framework usage
- Provide a clean API for common use cases
- Layer architecture (API facade over business logic)