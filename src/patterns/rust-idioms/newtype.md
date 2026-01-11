### Newtype Pattern

The Newtype pattern wraps an existing type in a tuple struct with a single field. This provides type safety, encapsulation, and the ability to implement traits on the wrapper type without affecting the original type.

**Benefits**:
- Zero-cost abstraction (compiled away at runtime)
- Type safety - distinct types even if they wrap the same underlying type
- Implement external traits on external types (orphan rule workaround)
- Hide implementation details
- Add semantic meaning to primitive types

```rust
{{#include newtype/src/main.rs}}
```

**Key Points**:
- The wrapper type is compiled away - zero runtime overhead
- Prevents accidentally mixing up values of the same underlying type
- Allows implementing traits on external types (e.g., implementing `Display` for `Vec<T>`)
- Use `#[repr(transparent)]` if you need guaranteed same memory layout as the inner type

**When to Use**:
- When you have multiple parameters of the same type (like `u64`)
- When you want to add semantic meaning to a type
- When you need to implement external traits on external types
- When you want to restrict what operations are valid on a type
