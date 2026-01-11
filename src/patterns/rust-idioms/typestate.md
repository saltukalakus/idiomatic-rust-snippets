### Type State Pattern

The Type State pattern encodes the state of an object in its type, making invalid state transitions a compile-time error. This leverages Rust's type system to provide compile-time guarantees about object states.

**Benefits**:
- Invalid state transitions are compile-time errors, not runtime errors
- Self-documenting API - the types show valid operations
- Zero runtime cost - states are types, not values
- No need for runtime state checks

```rust
{{#include typestate/src/main.rs}}
```

**Key Points**:
- States are zero-sized types (no runtime cost)
- State transitions consume `self` and return a new type
- Methods are only available in valid states
- `PhantomData` is used when the type parameter isn't stored
- The compiler enforces valid state transitions

**When to Use**:
- State machines (documents, connections, protocols)
- Builders with required fields
- Resource lifecycle management (open/closed, locked/unlocked)
- Any workflow with strict state transitions
