### 桥接（Bridge）模式

桥接模式将抽象与实现分离，使得两者可以独立变化。Rust 可通过 trait 对象或泛型参数来解耦抽象与实现。

```rust, editable
trait Implementor { fn do_it(&self); }
struct ConcreteImpl;
impl Implementor for ConcreteImpl { fn do_it(&self) { println!("doing"); } }

struct Abstraction<T: Implementor> { imp: T }
impl<T: Implementor> Abstraction<T> { fn new(imp: T) -> Self { Self { imp } } fn op(&self) { self.imp.do_it() } }
```
