### 代理（Proxy）模式

代理模式提供对象的占位或代表，可在访问真实对象前进行控制（如延迟加载、访问控制或缓存）。Rust 中可使用封装类型或智能指针实现代理。

```rust, editable
trait Subject { fn request(&self); }
struct Real; impl Subject for Real { fn request(&self) { println!("真实"); } }
struct Proxy { real: Real }
impl Subject for Proxy { fn request(&self) { println!("代理中"); self.real.request(); } }
```
