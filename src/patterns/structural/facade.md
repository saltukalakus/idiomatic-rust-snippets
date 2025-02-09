### Facade Design Pattern

The Facade pattern provides a simplified interface to a complex subsystem. It defines a higher-level interface that makes the subsystem easier to use.

```rust
{{#include facade/src/main.rs}}
```

1. **SubsystemA and SubsystemB**: Represent the complex subsystems with their own operations.
2. **Facade Struct**: Provides a simplified interface to interact with the subsystems.
3. **new() Method**: Initializes the subsystems.
4. **operation() Method**: Provides a higher-level interface to perform operations involving multiple subsystems.
5. **main() Function**: Demonstrates the usage of the facade to interact with the subsystems.

This example shows how the Facade pattern simplifies the interaction with complex subsystems by providing a unified interface.