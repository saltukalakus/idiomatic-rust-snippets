### Enum Polymorphism

Enum polymorphism uses Rust's algebraic data types with pattern matching to perform different operations on different types. This provides compile-time exhaustiveness checking and excellent performance.

**Benefits**:
- Exhaustive matching - compiler ensures all variants are handled
- Zero-cost abstraction - faster than dynamic dispatch
- Simple borrowing semantics
- Adding operations is straightforward

```rust
{{#include enum-polymorphism/src/main.rs}}
```

**Key Points**:
- The example defines `Shape` enum with `Circle`, `Rectangle`, and `Triangle` variants
- Each variant stores its specific data (radius, width/height, sides)
- `area()` and `perimeter()` methods use `match` to handle each variant differently
- Compiler ensures all variants are handled - adding a new variant causes compile errors until all matches updated
- In `main()`, shapes stored in `Vec<Shape>` and processed uniformly - no heap allocation or vtables needed

**When to Use**:
- Fixed set of types that need different behaviors
- Performance-critical code (no dynamic dispatch)
- Abstract syntax trees, expression evaluators
- When you control all the types (not for plugins)