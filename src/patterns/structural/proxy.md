### Proxy Design Pattern

The Proxy pattern provides a surrogate or placeholder for another object to control access to it. This can be useful for various purposes such as lazy initialization, access control, logging, etc.

```rust
{{#include proxy/src/main.rs}}
```

1. **Subject Trait**: Defines the common interface for `RealSubject` and `Proxy`.
2. **RealSubject Struct**: Implements the `Subject` trait and contains the actual business logic.
3. **Proxy Struct**: Contains a reference to `RealSubject` and implements the `Subject` trait to control access to `RealSubject`.
4. **Proxy::new() Method**: Creates a new instance of the `Proxy` with an instance of `RealSubject`.