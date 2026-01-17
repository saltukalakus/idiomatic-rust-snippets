### 抽象方法（Abstract Method）

抽象方法模式通过在基类或 trait 中声明方法签名来定义接口，子类或实现者负责具体实现。Rust 中等价的是定义 trait 并在类型上实现它们。

```rust, editable
trait Renderer { fn render(&self); }

struct Svg;
impl Renderer for Svg { fn render(&self) { println!("svg"); } }
```
