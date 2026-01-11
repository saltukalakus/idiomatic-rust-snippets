### Enum Polymorphism

Enum polymorphism uses Rust's algebraic data types (enums) with pattern matching to perform different operations on different types. This provides compile-time exhaustiveness checking, better performance, and clearer code.

```rust
{{#include enum-polymorphism/src/main.rs}}
```

**Key Benefits**:
1. **Exhaustive matching**: The compiler ensures all enum variants are handled in each match expression
2. **Performance**: Direct enum matching is faster than dynamic dispatch
3. **Clear ownership**: Simple borrowing semantics
4. **Easy to extend**: Just add a new method with a match expression
5. **Type safety**: Cannot accidentally miss a case - compiler enforces completeness

**When to Use Trait Objects Instead**:
- When you need to add operations from external crates (plugin systems)
- When the set of types is fixed but operations are frequently added by different modules
- When the enum variants would be too numerous or complex