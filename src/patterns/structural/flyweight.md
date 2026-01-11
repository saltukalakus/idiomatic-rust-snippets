### Flyweight Pattern

The flyweight pattern minimizes memory usage by sharing data among similar objects. Intrinsic (shared) state is separated from extrinsic (unique) state.

**Benefits**:
- Reduces memory usage when many similar objects exist
- Improves cache locality by sharing data
- Separates shared state from unique state
- Particularly effective with immutable shared data

```rust
{{#include flyweight/src/main.rs}}
```

**Key Points**:
- The example shows `Flyweight` struct storing shared intrinsic state (color, size)
- `FlyweightFactory` maintains `HashMap<String, Arc<Flyweight>>` to cache flyweights by key
- `get_flyweight()` checks if flyweight exists; if so, clones the `Arc`, otherwise creates new one
- In `main()`, multiple objects share same flyweight - requesting "red" twice returns same instance
- Memory saved by sharing common data; extrinsic state (position, context) passed when using flyweight

**When to Use**:
- Large numbers of similar objects consuming memory
- Object state can be divided into intrinsic (shared) and extrinsic (unique)
- Text rendering (shared font data), game entities (shared sprites)
- Immutable cached data shared across instances