### Message Passing

Message passing allows publishers to notify subscribers of events through channels. This decouples publishers from subscribers, making code more modular and thread-safe.

**Benefits**:
- Avoids shared mutable state - data sent through channels
- Natural thread safety - channels work across threads
- Clear ownership - subscribers own their receiving end
- Automatic cleanup when receivers are dropped

```rust
{{#include message-passing/src/main.rs}}
```

**Key Points**:
- The example shows two approaches: traditional observers with `Arc<dyn Observer>` and channel-based pub-sub
- Traditional: `Subject` maintains `Vec<Arc<dyn Observer>>`, calls `update()` on each during `notify()`
- Channel-based: `ChannelSubject` stores `Vec<Sender<String>>`, sends messages via `send()`
- Receivers automatically dropped when subscribers go out of scope - no manual cleanup
- Channel approach avoids trait objects and provides better thread safety

**When to Use**:
- Pub-sub patterns (event buses, notifications)
- Cross-thread communication
- Decoupling components that react to state changes
- When automatic cleanup on subscriber drop is desired