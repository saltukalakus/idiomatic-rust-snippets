### Adapter Design Pattern

The Adapter pattern allows incompatible interfaces to work together. This is useful when you want to use a class that doesn't have the exact interface you need.

```rust
{{#include adapter/src/main.rs}}
```

1. **Target Trait**: Defines the interface that the client expects.
2. **Adaptee Struct**: The existing class with a different interface that needs to be adapted.
3. **Adapter Struct**: Implements the `Target` trait and translates the interface of `Adaptee` to the `Target` interface.