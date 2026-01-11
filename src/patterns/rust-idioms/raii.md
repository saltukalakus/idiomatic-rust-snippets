### RAII (Resource Acquisition Is Initialization)

RAII is a fundamental pattern in Rust where resources (memory, files, locks, connections) are automatically cleaned up when their owner goes out of scope. This is implemented through Rust's ownership system and the `Drop` trait.

**Benefits**:
- No resource leaks - cleanup is guaranteed
- Exception-safe - cleanup happens even if panics occur
- No garbage collector needed
- Explicit resource lifetime tied to scope

```rust
{{#include raii/src/main.rs}}
```

**Key Points**:
- The example shows `DatabaseConnection` created in `new()` (prints "Opening"), automatically closed via `Drop` (prints "Closing")
- When `conn` goes out of scope (closing brace), `drop()` is called automatically
- Nested example: `conn2` dropped before `conn1` due to LIFO order (inner scopes first)
- `might_fail()` returns early with error, but connection still closes - `Drop` runs even on early returns
- No manual cleanup needed - Rust's ownership ensures `Drop::drop()` always runs

**Common RAII Types in std**:
- `File` - closes file handle on drop
- `MutexGuard` - releases lock on drop
- `Box`, `Vec`, `String` - deallocate memory on drop
- `Rc`, `Arc` - decrement reference count on drop
- `JoinHandle` - can be used to ensure thread cleanup

**When to Use**:
- Whenever you have a resource that needs cleanup
- File handles, network connections, locks
- Temporary state that must be restored
- Any acquire/release pattern
