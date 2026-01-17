### 享元（Flyweight）模式

享元模式通过共享相同的可重用对象实例来减少内存占用，通常把可共享的状态与不可共享的状态分离。Rust 中可使用缓存和 `Arc` 实现共享。

```rust, editable
use std::sync::Arc;
use std::collections::HashMap;

struct Flyweight { data: Arc<String> }

fn get_flyweight(cache: &mut HashMap<String, Arc<String>>, key: &str) -> Flyweight {
    let v = cache.entry(key.to_string()).or_insert_with(|| Arc::new(key.to_string()));
    Flyweight { data: Arc::clone(v) }
}
```
