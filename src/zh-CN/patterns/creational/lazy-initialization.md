### 惰性初始化（Lazy Initialization）

惰性初始化用于延迟创建昂贵的资源直到首次使用。在 Rust 中可使用 `lazy_static`、`once_cell` 或 `std::sync::OnceLock`（稳定后）实现线程安全的惰性单例。

```rust, editable
use once_cell::sync::Lazy;
static CONFIG: Lazy<String> = Lazy::new(|| {
    // 复杂初始化
    "config".to_string()
});

fn main() {
    println!("{}", *CONFIG);
}
```
