### 不必要的智能指针

当简单的引用或直接拥有值即可时使用 `Box<T>`、`Rc<T>` 或 `Arc<T>` 是浪费的。智能指针会带来额外的间接性和堆分配开销。它们适用于特定的所有权场景：递归类型、共享所有权或动态特征对象。

不必要地使用智能指针会使代码复杂并降低性能。除非有明确的间接需求，否则优先使用直接所有权或引用。

```rust, editable
struct Config {
    timeout: Box<u32>,      // 不必要的堆分配
    max_retries: Box<u32>,  // 不必要的堆分配
    endpoint: Box<String>,  // 双重堆分配
}

impl Config {
    fn new() -> Self {
        Config {
            timeout: Box::new(30),
            max_retries: Box::new(3),
            endpoint: Box::new("https://api.example.com".to_string()),
        }
    }
    
    fn display(&self) {
        println!("超时: {}s", self.timeout);
        println!("重试次数: {}", self.max_retries);
        println!("端点: {}", self.endpoint);
    }
}

fn main() {
    let config = Config::new();
    config.display();
    // 三次不必要的堆分配用于简单值
}
```

将原语类型装箱（如 `u32`）会为没有实际收益的情况增加堆分配开销。`Box<String>` 会造成双重间接，因为 `String` 本身已经在堆上分配。每次 `Box::new()` 都需要一次堆分配，导致内存布局碎片化，降低缓存效率并增加分配开销。

下面的示例使用直接所有权进行改进。

```rust, editable
struct Config {
    timeout: u32,
    max_retries: u32,
    endpoint: String,
}

impl Config {
    fn new() -> Self {
        Config {
            timeout: 30,
            max_retries: 3,
            endpoint: "https://api.example.com".to_string(),
        }
    }
    
    fn display(&self) {
        println!("超时: {}s", self.timeout);
        println!("重试次数: {}", self.max_retries);
        println!("端点: {}", self.endpoint);
    }
}

fn main() {
    let config = Config::new();
    config.display();
    // 仅为 String 产生一次堆分配
}
```

**关键改进**：
- 原语类型直接存储 —— 无需堆分配
- 更佳的内存局部性 —— 除 `String` 外字段均在栈上
- 代码更简单 —— 无需不必要的 `Box::new()` 或解引用
- 访问更快 —— 原语无指针间接访问
- 结构体大小更小 —— `Config` 更符合缓存友好性

