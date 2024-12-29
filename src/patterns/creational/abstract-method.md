### Abstract Factory

The Abstract Factory design pattern provides an interface for creating families of related or dependent objects without specifying their concrete classes. This pattern is particularly useful when the exact types and dependencies of the objects are not known until runtime.

Here is an example of the Abstract Factory Pattern:

```rust
{{#include abstract-method/src/main.rs}}
```

In this example, the `FurnitureFactory` trait defines methods for creating abstract products (`Chair` and `Sofa`). The `ModernFurnitureFactory` and `VictorianFurnitureFactory` structs implement this trait to create concrete products. The client code uses the factory to create and interact with the products without knowing their concrete types.