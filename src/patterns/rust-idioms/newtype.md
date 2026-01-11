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
- The example shows `UserId` and `AccountId` wrapping `u64` - prevents mixing them up accidentally
- Each newtype has `new()` constructor and `value()` accessor for the inner type
- `process_user()` takes specific types - compiler rejects `process_user(account, user)` (wrong order)
- Second example: `Meters` and `Kilometers` wrapping `f64` with `From` trait for conversions
- All wrapper types compiled away - zero runtime overhead, just compile-time safety

**When to Use**:
- When you have multiple parameters of the same type (like `u64`)
- When you want to add semantic meaning to a type
- When you need to implement external traits on external types
- When you want to restrict what operations are valid on a type
