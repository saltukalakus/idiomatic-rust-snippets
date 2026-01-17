### 外观（Facade）模式

外观为复杂子系统提供一个简化接口，使客户端更容易使用。在 Rust 中可以使用简单的模块函数或聚合类型来实现外观。

```rust, editable
struct SubA; impl SubA { fn a(&self) {} }
struct SubB; impl SubB { fn b(&self) {} }

struct Facade { a: SubA, b: SubB }
impl Facade { fn do_all(&self) { self.a.a(); self.b.b(); } }
```
