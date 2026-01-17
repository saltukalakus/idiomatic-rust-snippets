# 什么是类型别名（type alias）？

在 Rust 中，类型别名允许你为现有类型创建一个新名称。这可以提高代码的可读性并简化复杂类型的管理。使用 `type` 关键字创建类型别名。

### 类型别名的好处

- 可读性更高：通过为类型赋予有意义的名称，使代码更易理解。
- 简化代码：类型别名可以简化复杂的类型签名，使代码更易编写和阅读。
- 保持一致性：使用类型别名有助于在代码库中保持一致性。

```rust, editable
// type NewName = ExistingType;

type Kilometers = i32;

fn main() {
    let distance: Kilometers = 100;
    println!("Distance: {} km", distance);
}
```

`Kilometers` 是 `i32` 的类型别名，清楚地表明 `distance` 变量表示以公里为单位的距离。

```rust, editable
struct Point<T> {
    x: T,
    y: T,
}

type IntPoint = Point<i32>;
type FloatPoint = Point<f64>;

fn main() {
    let int_point: IntPoint = Point { x: 5, y: 10 };
    let float_point: FloatPoint = Point { x: 1.0, y: 4.0 };

    println!("IntPoint: ({}, {})", int_point.x, int_point.y);
    println!("FloatPoint: ({}, {})", float_point.x, float_point.y);
}
```

`IntPoint` 和 `FloatPoint` 分别是 `Point<i32>` 和 `Point<f64>` 的类型别名，这样更清晰地表达了所使用的点类型。

类型别名对于复杂类型（例如函数指针或嵌套类型）特别有用。

```rust, editable
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // Execute the function
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("你好，世界！"))
}

fn main() {
    let f: Thunk = returns_long_type();
    takes_long_type(f);
}
```

`type Thunk = Box<dyn Fn() + Send + 'static>;` 创建了名为 `Thunk` 的类型别名。<br/>
**Box**：`Box` 是一种智能指针，在堆上分配数据。<br/>
**dyn Fn()**：`dyn Fn()` 是表示不接受参数且不返回值的闭包的特质对象。<br/>
**Send**：`Send` 特质表示该闭包可以跨线程边界传输。<br/>
**'static**：`'static` 生命周期表示该闭包可以存活整个程序的持续时间。
