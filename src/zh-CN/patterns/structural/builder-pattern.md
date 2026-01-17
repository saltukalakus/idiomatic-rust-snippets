### Builder Pattern (Method Chaining)

The builder pattern with method chaining allows composing behavior by chaining method calls. Each method modifies and returns `self`, enabling fluent APIs and zero-cost abstractions.

**Benefits**:
- Zero heap allocations - entirely stack-based
- Compile-time dispatch - no vtable overhead
- Fluent, readable API
- Type-safe composition at compile time

```rust, editable
{{#include builder-pattern/src/main.rs}}
```

**Key Points**:
- The example compares trait object approach (`Box<dyn Text>`) with method chaining
- Trait object version: `Bold` wraps `Italic` which wraps `PlainText` - each allocation on heap
- Method chaining version: `TextBuilder` has `content`, `bold`, and `italic` fields
- Each builder method (`bold()`, `italic()`) sets a flag and returns `self`
- `render()` applies formatting based on flags - all stack-allocated, zero heap overhead

**When to Use**:
- Composing optional features or behaviors
- Building fluent APIs
- Zero-cost abstractions over runtime dispatch
- When the set of behaviors is known at compile time