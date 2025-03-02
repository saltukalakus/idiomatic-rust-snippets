### Bridge Design Pattern

The Bridge pattern is used to separate the abstraction from its implementation, allowing them to vary independently. It is useful when both the class and what it does vary often.

```rust
{{#include bridge/src/main.rs}}
```

1. **Shape Trait**: Defines the abstraction with a `draw` method.
2. **Color Trait**: Defines the implementor with a `fill` method.
3. **Red and Blue Structs**: Concrete implementors of the `Color` trait.
4. **Circle and Square Structs**: Refined abstractions that implement the `Shape` trait and use a `Color` to fill the shape.