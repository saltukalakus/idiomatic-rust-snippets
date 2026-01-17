````markdown
### 理解 Rust 中的 `Self`

在 Rust 中，`Self` 是一个特殊的类型别名，指代当前 trait 或实现块的类型。它通常在 trait 定义和实现中用来指代实现该 trait 的类型。

```rust, editable
trait MyTrait {
    fn new() -> Self;
    fn describe(&self) -> String;
}

struct MyStruct {
    name: String,
}

impl MyTrait for MyStruct {
    fn new() -> Self {
        Self {
            name: String::from("MyStruct"),
        }
    }

    fn describe(&self) -> String {
        format!("这是 {}", self.name)
    }
}

fn main() {
    let instance = MyStruct::new();
    println!("{}", instance.describe());
}
```

- `MyTrait` trait 定义了两个方法：`new` 和 `describe`。
- `new` 方法返回一个实现该 trait 的类型的实例，用 `Self` 表示。
- `describe` 方法接受一个对 `self` 的引用，并返回一个描述字符串。
- `MyStruct` 结构体实现了 `MyTrait` trait，在实现中使用 `Self` 来指代它自己的类型。

这使得代码更加灵活和可重用，因为 `Self` 可以适应实现该 trait 的类型。
````
