### Type State Pattern

The Type State pattern encodes the state of an object in its type, making invalid state transitions a compile-time error. This leverages Rust's type system to provide compile-time guarantees about object states.

**Benefits**:
- Invalid state transitions are compile-time errors, not runtime errors
- Self-documenting API - the types show valid operations
- Zero runtime cost - states are types, not values
- No need for runtime state checks

```rust, editable
{{#include typestate/src/main.rs}}
```

**Key Points**:
- The example shows `Document<Draft>`, `Document<Review>`, and `Document<Published>` as separate types
- `Document<Draft>` has `write()` and `submit_for_review()` methods; others don't
- `submit_for_review()` consumes `Document<Draft>` and returns `Document<Review>` - state transition at type level
- Calling `doc.write()` on `Document<Published>` is a compile error - method doesn't exist for that type
- Second example: `ConnectionBuilder` with `build()` only available when both host AND port are set
- `PhantomData<State>` marks the type parameter without storing it - zero runtime size

**When to Use**:
- State machines (documents, connections, protocols)
- Builders with required fields
- Resource lifecycle management (open/closed, locked/unlocked)
- Any workflow with strict state transitions
