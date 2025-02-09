### Iterator Pattern

The iterator pattern is a design pattern that provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation. In Rust, the iterator pattern is implemented using the `Iterator` trait.

```rust
{{#include iterator/src/main.rs}}
```

Rust's standard library provides many useful methods for iterators. In this example, we use the `zip`, `skip`, `map`, `filter`, and `sum` methods to perform various operations on the iterator.

The iterator pattern in Rust is a powerful and flexible way to work with sequences of data. By implementing the `Iterator` trait, you can create custom iterators and take advantage of the many methods provided by the standard library to manipulate and process data efficiently.