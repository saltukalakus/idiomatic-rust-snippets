### Composite Design Pattern

The Composite pattern allows you to compose objects into tree structures to represent part-whole hierarchies. It lets clients treat individual objects and compositions of objects uniformly.

```rust
{{#include composite/src/main.rs}}
```

1. **Component Trait**: Defines the common interface for all components, both simple and complex.
2. **Leaf Struct**: Represents the leaf objects in the composition. Implements the `Component` trait.
3. **Composite Struct**: Represents the composite objects that can have children. Implements the `Component` trait and provides methods to add children.