### Adapter Design Pattern

The Adapter pattern allows incompatible interfaces to work together. This is useful when you want to use a class that doesn't have the exact interface you need.

Here is an example of the Adapter Design Pattern:

```rust
{{#include adapter/src/main.rs}}
```

1. **Target Trait**: Defines the interface that the client expects.
2. **Adaptee Struct**: The existing class with a different interface that needs to be adapted.
3. **Adapter Struct**: Implements the `Target` trait and translates the interface of `Adaptee` to the `Target` interface.
4. **main() Function**: Demonstrates the usage of the adapter to make `Adaptee` compatible with the `Target` interface.

This example shows how the Adapter pattern can be used to make two incompatible interfaces work together.
