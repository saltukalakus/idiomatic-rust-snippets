### Message Passing

Message passing allows an object (the subject) to notify multiple dependents (subscribers) of state changes. This pattern decouples publishers from subscribers, making code more modular and easier to test.

**Why Message Passing Works Well in Rust**:
- Avoids shared mutable state - data is sent through channels
- Natural thread safety - channels can be used across threads
- Clear ownership - subscribers own their receiving end
- Flexible - subscribers can be added/removed dynamically

```rust
{{#include message-passing/src/main.rs}}
```

**Two Approaches Shown**:

1. **With Callbacks** (Arc<dyn Observer>): Uses shared ownership for observers. Observers can be subscribed and unsubscribed. Requires heap allocation and reference counting.

2. **With Channels**: Uses built-in message passing with `mpsc::channel`:
   - Automatic cleanup when receivers are dropped
   - Thread-safe by default
   - Can be moved across threads
   - No trait objects needed
   - Clear ownership semantics

**Recommendations**:
- **Use channels** for pub-sub patterns (crossbeam-channel or tokio channels for async)
- **Use callbacks** when you need dynamic subscription/unsubscription
- **Consider async streams** for async code (futures::Stream, tokio::watch)
- Avoid circular references - use `Weak` if needed