### Enum Polymorphism (Rust Alternative to Visitor)

**Traditional Pattern**: Visitor Pattern  
**Rust Idiomatic Approach**: Enums with Pattern Matching

The traditional OOP visitor pattern uses double dispatch to add operations to object structures. However, in Rust, this pattern is often **unnecessary and awkward** due to borrowing restrictions and the power of enums with pattern matching.

**Idiomatic Rust Approach**: Use enums and pattern matching instead of the visitor pattern. This provides compile-time exhaustiveness checking, better performance, and clearer code.

```rust
{{#include enum-polymorphism/src/main.rs}}
```

**Why This Is Better Than Traditional Visitor**:
1. **Exhaustive matching**: The compiler ensures all enum variants are handled in each match expression
2. **No trait objects**: Direct enum matching is faster than dynamic dispatch through `dyn Trait`
3. **Clear ownership**: No complex `&mut` borrowing through trait methods
4. **Adding operations is easy**: Just add a new method with a match expression
5. **Type safety**: Cannot accidentally miss a case - compiler enforces completeness

**When Traditional Visitor Makes Sense**:
- When you need to add operations from external crates (plugin systems)
- When the set of types is fixed but operations are frequently added by different modules
- When working with existing OOP-style code that you cannot refactor

**Key Takeaway**: In Rust, prefer enums + pattern matching over the visitor pattern. Rust's algebraic data types make the visitor pattern largely obsolete.