### Lazy Initialization

Lazy initialization ensures that a value is initialized only once on first access. This is useful for global state like logging, metrics, or application-wide configuration that should be initialized exactly once, even in multi-threaded environments.

**Benefits**:
- Thread-safe initialization without unsafe code
- Deferred initialization until first use
- Guaranteed single initialization even with concurrent access
- Zero runtime cost after first initialization

```rust, editable
{{#include lazy-initialization/src/main.rs}}
```

**Key Points**:
- The example shows `AppConfig` as a global singleton accessed via `get_config()`
- `OnceLock` ensures `AppConfig` is initialized only once on first access using `get_or_init()`
- `Arc<Mutex<AppConfig>>` wraps the config for thread-safe shared access and mutation
- Main thread and spawned threads all access the same instance - incrementing from 0 to 3
- After initialization, subsequent calls to `get_config()` return the cached static reference with zero overhead

**When to Use**:
- Global state like logging, metrics, or configuration
- Expensive initialization that should be deferred
- Singleton-like patterns where you need exactly one instance
- Thread-safe lazy evaluation