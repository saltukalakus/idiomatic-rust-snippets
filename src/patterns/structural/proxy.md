### Proxy Pattern

The proxy pattern provides a surrogate or placeholder for another object, controlling access to it. The proxy has the same interface as the real object.

**Benefits**:
- Controls access to the real object
- Can add functionality (logging, caching, lazy loading)
- Real object can be remote, expensive to create, or needs protection
- Transparent to clients - same interface as real object

```rust, editable
{{#include proxy/src/main.rs}}
```

**Key Points**:
- The example defines `Subject` trait with `request()` method
- `RealSubject` implements the actual business logic
- `Proxy` wraps `RealSubject` and implements same `Subject` trait
- In `Proxy::request()`, proxy adds logging before/after calling `real_subject.request()`
- Client code works with `Subject` trait - can use proxy or real subject interchangeably

**When to Use**:
- Lazy initialization of expensive objects
- Access control or authentication
- Logging, caching, or monitoring object usage
- Remote objects (network proxies), smart references