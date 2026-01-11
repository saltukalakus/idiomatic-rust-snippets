### Builder Pattern

The builder pattern constructs complex objects step-by-step, separating construction logic from the final object representation. This is one of the most idiomatic patterns in Rust.

**Benefits**:
- Construct complex objects with many optional parameters
- Readable, self-documenting API
- Immutable objects with flexible construction
- Compile-time validation of required fields possible

```rust
{{#include builder/src/main.rs}}
```

**Key Points**:
- The example shows `HouseBuilder` with methods like `bedrooms()`, `bathrooms()`, `garage()` that return `self`
- Each method modifies the builder and returns it, enabling chaining: `.bedrooms(3).bathrooms(2).garage(true)`
- `build()` consumes the builder and constructs the final `House` struct
- Default values can be set in `new()` - only specify what differs from defaults
- Very common in Rust APIs (e.g., `std::process::Command`, HTTP client builders)

**When to Use**:
- Objects with many optional configuration parameters
- Complex construction logic that shouldn't be in constructors
- When you want an immutable object with flexible construction
- APIs that prioritize readability and discoverability