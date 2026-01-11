### Lazy Initialization (Rust Alternative to Singleton)

**Traditional Pattern**: Singleton Pattern  
**Rust Idiomatic Approach**: Lazy Initialization with `OnceLock` or `LazyLock`

The traditional Singleton pattern ensures a class has only one instance. In Rust, this is achieved safely using `OnceLock` or `LazyLock` instead of unsafe static mut patterns common in other languages.

**Important Note**: While singletons are a classic pattern, in Rust it's often better to use dependency injection or pass configuration explicitly. Use lazy initialization only when truly necessary for global state like logging, metrics, or application-wide configuration.

```rust
{{#include lazy-initialization/src/main.rs}}
```

**Key Points**:
1. **OnceLock**: Provides safe, lazy initialization without unsafe code. The value is initialized exactly once on first access.
2. **Arc<Mutex<T>>**: Enables thread-safe shared ownership and interior mutability.
3. **No unsafe code**: Unlike traditional singleton implementations, this approach uses only safe Rust.
4. **Thread-safe**: Multiple threads can safely access and modify the singleton through the Mutex.
5. **Zero-cost after initialization**: After the first call, `get_or_init` just returns the cached reference.

**Idiomatic Alternatives**:
- For read-only config: Use `OnceLock<T>` or `LazyLock<T>` directly without Arc/Mutex
- For dependency injection: Pass configuration through function parameters or struct fields
- For async code: Use `tokio::sync::OnceCell` for async initialization