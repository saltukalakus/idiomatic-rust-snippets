### Builder Pattern (Rust's Idiomatic Alternative to Decorator)

**Traditional Pattern**: Decorator Pattern  
**Rust Idiomatic Approach**: Builder Pattern with Method Chaining

The traditional Decorator pattern allows behavior to be added to objects dynamically. While the traditional OOP approach works in Rust, there are more idiomatic alternatives that leverage Rust's ownership system and zero-cost abstractions.

**Why Builder Pattern is Better in Rust**: The builder pattern with method chaining provides the same composability as decorators but with zero heap allocations and compile-time dispatch.

```rust
{{#include builder-pattern/src/main.rs}}
```

**Two Approaches Shown**:

1. **Traditional Decorator** (Box<dyn Trait>):
   - Uses trait objects and heap allocation
   - Requires dynamic dispatch (runtime cost)
   - Each decorator wraps the previous one
   - Familiar to developers from OOP backgrounds
   - **Downside**: Multiple heap allocations and pointer indirection

2. **Idiomatic Rust** (Builder/Newtype Pattern):
   - Methods consume and return `self`, allowing chaining
   - Zero heap allocations (stack-based)
   - All dispatch resolved at compile time
   - More performant and cleaner API
   - **Advantage**: Follows Rust's builder pattern idiom

**Other Rust Alternatives**:
- **Generics with type parameters**: For compile-time decoration without trait objects
- **Extension traits**: Add methods to existing types
- **Procedural macros**: For complex compile-time wrapping

**When to Use Each**:
- Use **traditional decorator** when you need runtime composition with unknown decorator combinations
- Use **builder pattern** for known set of optional features that can be compiled away
- Use **generics** when decorator types are known at compile time

**Key Takeaway**: In Rust, the builder pattern with method chaining is often more idiomatic and performant than the traditional decorator pattern.