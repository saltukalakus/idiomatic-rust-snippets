### Builder Pattern

The builder pattern is a creational design pattern that allows constructing complex objects step by step. It separates the construction of a complex object from its representation, allowing the same construction process to create different representations.

Here is an example:

```rust
{{#include builder/src/main.rs}}
```

In this example, `HouseBuilder` is used to construct a `House` object step by step. The builder provides methods to set the properties of the house and a `build` method to create the final `House` object.