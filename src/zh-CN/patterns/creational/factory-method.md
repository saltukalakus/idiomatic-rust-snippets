### 工厂方法（Factory Method）

工厂方法通过定义创建对象的接口，让子类或实现者决定实例化哪一个类。Rust 中通常使用 `trait` 返回 trait 对象或枚举来表示多态构造。

```rust, editable
trait Product { fn name(&self) -> &str; }

struct ConcreteA;
impl Product for ConcreteA { fn name(&self) -> &str { "A" } }

fn create(kind: &str) -> Box<dyn Product> {
    match kind {
        "A" => Box::new(ConcreteA),
        _ => Box::new(ConcreteA),
    }
}
```
