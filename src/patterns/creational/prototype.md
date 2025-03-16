### Prototype Pattern

The prototype pattern is a creational pattern that allows cloning of objects, even complex ones, without coupling to their specific classes. This pattern is particularly useful when the cost of creating a new instance of a class is expensive or complicated.

We can implement the pattern using the `Clone` trait.

```rust
{{#include prototype/src/main.rs}}
```

- We define a `Prototype` struct with two fields.
- We derive the `Clone` trait for the `Prototype` struct, which provides the `clone` method.
- We implement a `clone_prototype` method that clones the current instance.
- In the `main` function, we create an instance of `Prototype` and then clone it using the `clone_prototype` method.