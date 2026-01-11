### Builder Pattern

The builder pattern uses method chaining to construct objects step-by-step with various configurations. Each method consumes `self` and returns a modified version, allowing fluent and flexible object construction.

```rust
{{#include builder-pattern/src/main.rs}}
```

**Two Approaches Shown**:

1. **With Trait Objects** (Box<dyn Trait>):
   - Uses trait objects and heap allocation
   - Requires dynamic dispatch (runtime cost)
   - Each wrapper wraps the previous one
   - Multiple heap allocations and pointer indirection

2. **With Method Chaining** (Builder Pattern):
   - Methods consume and return `self`, allowing chaining
   - Zero heap allocations (stack-based)
   - All dispatch resolved at compile time
   - More performant and cleaner API

**Other Approaches**:
- **Generics with type parameters**: For compile-time composition without trait objects
- **Extension traits**: Add methods to existing types
- **Procedural macros**: For complex compile-time wrapping

**When to Use Each**:
- Use **trait objects** when you need runtime composition with unknown combinations
- Use **method chaining** for known set of optional features that can be compiled away
- Use **generics** when types are known at compile time