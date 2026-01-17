### 访问者（Visitor）模式

访问者模式用于将操作分离到对象结构之外，便于在不修改现有对象的情况下新增操作。Rust 中可使用枚举和 match 或 trait 组合来实现类似行为。

```rust, editable
trait Visitor { fn visit_number(&mut self, n: i32); }

enum Expr { Num(i32), Add(Box<Expr>, Box<Expr>) }

fn accept<V: Visitor>(expr: &Expr, v: &mut V) {
    match expr {
        Expr::Num(n) => v.visit_number(*n),
        Expr::Add(l, r) => { accept(l, v); accept(r, v); }
    }
}
```
