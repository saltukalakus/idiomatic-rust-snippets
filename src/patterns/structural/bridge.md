### Bridge Pattern

The bridge pattern separates an abstraction from its implementation, allowing both to vary independently. This avoids a proliferation of classes when both abstraction and implementation have multiple variants.

**Benefits**:
- Decouples abstraction from implementation
- Both can be extended independently
- Reduces the number of classes needed
- Implementation details hidden from clients

```rust
{{#include bridge/src/main.rs}}
```

**Key Points**:
- The example defines `Shape` trait (abstraction) and `Color` trait (implementation)
- `Circle` and `Square` implement `Shape`, each holding a `Box<dyn Color>`
- `Red` and `Blue` implement `Color` trait with different fill behaviors
- In `main()`, shapes created with different colors: red circle, blue square
- When `draw()` is called, shape delegates to its color's `fill()` method - shape and color vary independently

**When to Use**:
- Both abstraction and implementation have multiple variants
- Want to avoid Cartesian product of classes (N abstractions × M implementations)
- Platform-specific implementations (OS, rendering backends)
- Database drivers with multiple query languages and connection types