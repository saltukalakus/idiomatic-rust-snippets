### HashMap

`HashMap` 是一个存储键值对的集合，您可以通过其关联的键来查找值。

```rust, editable
use std::collections::HashMap;

fn main() {
    // 创建一个新的 HashMap
    let mut scores = HashMap::new();

    // 插入一些键值对
    scores.insert("Blue", 10);
    scores.insert("Red", 50);

    // 获取一个值
    match scores.get("Blue") {
        Some(score) => println!("Blue 队的得分: {}", score),
        None => println!("未找到 Blue 队"),
    }

    // 更新一个值
    scores.insert("Blue", 25); // 这将覆盖之前的值

    // 仅当键不存在时插入
    scores.entry("Yellow").or_insert(5);

    // 遍历键值对
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

常用操作：
- `new()`: 创建一个空的 HashMap
- `insert()`: 添加一个键值对
- `get()`: 通过键检索一个值
- `entry()`: 获取用于原地操作的条目
- `remove()`: 移除一个键值对
