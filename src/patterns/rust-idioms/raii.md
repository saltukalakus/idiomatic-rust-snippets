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
- Resources are acquired in constructors (`new`)
- Resources are released in `Drop::drop()`
- Drop is called automatically when value goes out of scope
- Drop order is LIFO (Last In, First Out) for nested resources
- Works correctly with `?` operator and early returns

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
