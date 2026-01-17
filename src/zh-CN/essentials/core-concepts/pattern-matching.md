### 模式匹配

模式匹配是一项强大的功能，允许你匹配复杂数据结构并根据其形状执行不同的操作。

### 匹配字面量

```rust, editable
let x = 1;

match x {
    1 => println!("一！"),
    2 => println!("二！"),
    _ => println!("其他！"),
}
```

### 匹配多个模式

```rust, editable
let x = 1;

match x {
    1 | 2 => println!("一或二！"),
    3 => println!("三！"),
    _ => println!("其他！"),
}
```

### 解构结构体

```rust, editable
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x: 0, y } => println!("在 y 轴，坐标 {}", y),
    Point { x, y: 0 } => println!("在 x 轴，坐标 {}", x),
    Point { x, y } => println!("不在任一轴：({}, {})", x, y),
}
```

### 解构枚举

```rust, editable
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => println!("Quit 变体没有可解构的数据。"),
    Message::Move { x, y } => println!("在 x 方向移动 {}，在 y 方向移动 {}", x, y),
    Message::Write(text) => println!("文本消息：{}", text),
    Message::ChangeColor(r, g, b) => println!("将颜色改为红 {}、绿 {}、蓝 {}", r, g, b),
}
```

### 在模式中忽略值

```rust, editable
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("一些数字：{}, {}, {}", first, third, fifth);
    },
}
```

### 匹配值的范围

```rust, editable
let x = 5;

match x {
    1..=5 => println!("一到五"),
    _ => println!("其他"),
}
```

### 使用 `if` 守卫

```rust, editable
let x = Some(4);

match x {
    Some(n) if n < 5 => println!("小于五：{}", n),
    Some(n) => println!("{}", n),
    None => (),
}
```
