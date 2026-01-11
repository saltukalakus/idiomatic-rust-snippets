### Message Passing (Rust Alternative to Observer)

**Traditional Pattern**: Observer Pattern  
**Rust Idiomatic Approach**: Channels and Message Passing

The traditional observer pattern allows an object (the subject) to notify multiple dependents (observers) of state changes. In Rust, this pattern requires careful handling of ownership and lifetimes, unlike in garbage-collected languages.

**Challenges with Traditional Observer in Rust**:
- Shared mutable state conflicts with Rust's borrowing rules
- Observers need to be removable, requiring careful ownership design
- Circular references must be avoided

**Rust's Solution**: Use channels for message passing instead of callback-based observers.

```rust
{{#include message-passing/src/main.rs}}
```

**Two Approaches Shown**:

1. **Traditional Observer with Arc**: Uses `Arc<dyn Observer>` for shared ownership. Observers can be subscribed and unsubscribed. Works but requires heap allocation and reference counting.

2. **Channel-based (Idiomatic)**: Uses Rust's built-in message passing with `mpsc::channel`. This is more idiomatic and flexible:
   - Automatic cleanup when receivers are dropped
   - Naturally thread-safe
   - Can be moved across threads
   - No trait objects needed
   - Follows Rust's "message passing over shared state" philosophy

**Recommendations**:
- **Use channels** for most observer-like patterns (crossbeam-channel or tokio channels for async)
- **Use callbacks** with `Arc` when you need dynamic subscription/unsubscription
- **Consider async streams** for async code (futures::Stream, tokio::watch)
- Avoid circular references - use `Weak` if needed

**Key Takeaway**: Rust's channels are often a better choice than traditional observer patterns.