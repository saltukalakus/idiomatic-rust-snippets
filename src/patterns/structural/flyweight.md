### Flyweight Design Pattern

The Flyweight pattern is used to minimize memory usage by sharing as much data as possible with similar objects. It is particularly useful when dealing with a large number of objects that have some shared state.

```rust
{{#include flyweight/src/main.rs}}
```

1. **Flyweight Struct**: Contains the shared state that can be reused.
2. **FlyweightFactory Struct**: Manages the creation and reuse of Flyweight objects.
3. **get_flyweight() Method**: Returns an existing Flyweight if available, otherwise creates a new one.