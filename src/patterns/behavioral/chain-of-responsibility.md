### Chain of Responsibility Pattern

The chain of responsibility pattern is a behavioral design pattern that allows an object to pass a request along a chain of potential handlers until the request is handled.

```rust
{{#include chain-of-responsibility/src/main.rs}}
```

- `Handler` is a trait that defines the interface for handling requests.
- `BaseHandler` is a base struct that implements the common behavior for setting the next handler and passing the request along the chain.
- `ConcreteHandlerA` and `ConcreteHandlerB` are concrete implementations of the `Handler` trait that handle specific requests.
- The `main` function sets up the chain and tests the handlers with different requests.