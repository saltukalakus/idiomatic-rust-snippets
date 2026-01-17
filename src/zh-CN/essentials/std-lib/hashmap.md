### HashMap

`HashMap<K, V>` 是 Rust 标准库中的哈希表，提供基于键的快速查找、插入与删除操作。

```rust, editable
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    if let Some(score) = scores.get("Blue") {
        println!("Blue: {}", score);
    }

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }
}
```

常用操作：`insert`、`get`、`remove`、迭代、以及使用 `entry` API 来处理默认值或修改已有项。

```rust, editable
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", counts);
}
```
