### 适配器（Adapter）模式

适配器用于将一个接口转换为另一种接口，使得原本不兼容的类型能够协同工作。在 Rust 中通常通过封装和实现 trait 来实现适配器。

```rust, editable
trait Target { fn request(&self) -> String; }

struct Adaptee;
impl Adaptee { fn specific(&self) -> String { "specific".into() } }

struct Adapter(Adaptee);
impl Target for Adapter { fn request(&self) -> String { self.0.specific() } }
```
