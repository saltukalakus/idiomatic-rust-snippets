### `impl` - 实现块

`impl` 块用于为类型实现方法与关联函数，也用于为 trait 提供实现：

```rust, editable
struct Point { x: i32, y: i32 }

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p = Point::new(3, 4);
    println!("distance: {}", p.dist_from_origin());
}
```

你也可以为 trait 提供 `impl`：

```rust, editable
trait Area { fn area(&self) -> f64; }

impl Area for Point {
    fn area(&self) -> f64 { 0.0 }
}
```
