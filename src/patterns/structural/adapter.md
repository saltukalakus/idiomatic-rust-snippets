### Adapter Pattern

The adapter pattern allows incompatible interfaces to work together by wrapping an existing interface with a new one. This is useful for integrating third-party libraries or legacy code.

**Benefits**:
- Integrate incompatible interfaces without modifying them
- Reuse existing functionality with different interfaces
- Decouple client code from specific implementations
- Wrap external types to implement local traits

```rust
{{#include adapter/src/main.rs}}
```

**Key Points**:
- The example defines `Target` trait with `request()` method
- `Adaptee` has `specific_request()` method with different name/signature
- `Adapter` wraps `Adaptee` and implements `Target` trait
- In `request()` implementation, adapter calls `self.adaptee.specific_request()` to delegate
- Client code in `main()` works with `Target` trait, unaware of `Adaptee` implementation details

**When to Use**:
- Integrating third-party libraries with incompatible APIs
- Making legacy code work with new interfaces
- Implementing traits on external types (orphan rule workaround)
- Creating a uniform interface for multiple types