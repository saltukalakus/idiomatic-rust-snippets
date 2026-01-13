### Chain of Responsibility Pattern

The chain of responsibility passes a request along a chain of handlers. Each handler decides whether to process the request or pass it to the next handler.

**Benefits**:
- Decouples request senders from receivers
- Dynamic handler chain configuration
- Each handler has single responsibility
- Easy to add or reorder handlers

```rust, editable
{{#include chain-of-responsibility/src/main.rs}}
```

**Key Points**:
- The example defines `Handler` trait with `handle(&mut self, request: &str)` method
- `BaseHandler` stores `Option<Box<dyn Handler>>` for the next handler in chain
- `ConcreteHandlerA` handles requests starting with 'A', otherwise passes to next
- `ConcreteHandlerB` handles requests starting with 'B', otherwise passes to next
- In `main()`, chain is built: Handler A → Handler B, requests flow through until matched

**When to Use**:
- Multiple objects might handle a request
- Handler set should be determined dynamically
- Middleware pipelines (logging, auth, validation)
- Event handling systems with fallback behaviors