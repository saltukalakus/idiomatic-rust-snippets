### Lazy Initialization

Lazy initialization ensures that a value is initialized only once on first access. This is useful for global state like logging, metrics, or application-wide configuration that should be initialized exactly once, even in multi-threaded environments.

```rust
{{#include lazy-initialization/src/main.rs}}
```

**Key Points**:
1. **OnceLock**: Provides safe, lazy initialization. The value is initialized exactly once on first access.
2. **Arc<Mutex<T>>**: Enables thread-safe shared ownership and interior mutability.
3. **Thread-safe**: Multiple threads can safely access and modify the value through the Mutex.
4. **Zero-cost after initialization**: After the first call, `get_or_init` just returns the cached reference.

**Idiomatic Alternatives**:
- For read-only config: Use `OnceLock<T>` or `LazyLock<T>` directly without Arc/Mutex
- For dependency injection: Pass configuration through function parameters or struct fields
- For async code: Use `tokio::sync::OnceCell` for async initialization