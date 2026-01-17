### 装饰器（Decorator）模式

装饰器用于动态地给对象添加行为，而无需修改原始类型。Rust 中可用包装器类型（wrappers）或组合来实现装饰器。

```rust, editable
trait Component { fn op(&self); }
struct Concrete;
impl Component for Concrete { fn op(&self) { println!("core"); } }

struct Decorator<T: Component> { inner: T }
impl<T: Component> Component for Decorator<T> { fn op(&self) { println!("before"); self.inner.op(); println!("after"); } }
```
