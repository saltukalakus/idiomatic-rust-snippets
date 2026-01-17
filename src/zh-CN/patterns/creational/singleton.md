### 单例（Singleton）模式

单例模式确保某个类型只有一个实例，并提供全局访问点。Rust 更推荐使用线程安全的全局静态（例如 `once_cell::sync::Lazy` 或 `std::sync::OnceLock`）而不是传统的单例。

```rust, editable
use once_cell::sync::Lazy;
static INSTANCE: Lazy<MyType> = Lazy::new(|| MyType::new());

struct MyType; impl MyType { fn new() -> Self { MyType } }

fn main() { let _ = &*INSTANCE; }
```
