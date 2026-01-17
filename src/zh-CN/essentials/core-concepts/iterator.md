### 迭代器

迭代器是 Rust 处理中元素序列时最强大的特性之一。任何实现了 `Iterator` 特质的类型都可以作为迭代器，允许你逐个处理序列中的元素。迭代器是惰性的，这意味着在被消费前不会执行任何工作。

### `Iterator` 特质

Rust 迭代器系统的核心是 `Iterator` 特质，它要求实现 `next` 方法：

```rust, editable
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 创建迭代器
    let mut iter = numbers.iter();
    
    // 手动调用 next
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    
    // 在 for 循环中使用（会消费迭代器）
    for num in numbers.iter() {
        println!("{}", num);
    }
}
```

### 迭代器适配器

迭代器适配器可以将一个迭代器转换为另一个。它们是惰性的，直到调用消费方法才会执行。

```rust, editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 链接多个适配器
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)           // 每个元素乘以 2
        .filter(|x| *x > 5)       // 仅保留大于 5 的元素
        .collect();               // 消费并收集到 Vec
    
    println!("{:?}", result); // [6, 8, 10]
}
```

### 常用迭代器方法

**适配器（返回新的迭代器）**：
- `map()`: 转换每个元素
- `filter()`: 保留满足谓词的元素
- `take()`: 取前 n 个元素
- `skip()`: 跳过前 n 个元素
- `enumerate()`: 为元素添加索引
- `zip()`: 合并两个迭代器
- `chain()`: 连接迭代器
- `flat_map()`: 映射并扁平化嵌套结构

**消费者（产生最终值）**：
- `collect()`: 收集元素到集合
- `sum()`, `product()`: 计算和或积
- `fold()`: 归约为单个值
- `find()`: 查找第一个匹配元素
- `any()`, `all()`: 检查是否有/全部匹配谓词
- `count()`: 统计元素数量
- `nth()`: 获取第 n 个元素
- `for_each()`: 为每个元素执行闭包

### 迭代器类型

```rust, editable
fn main() {
    let v = vec![1, 2, 3];
    
    // iter() - 不可变借用每个元素
    for item in v.iter() {
        println!("{}", item); // item 是 &i32
    }
    
    // iter_mut() - 可变借用每个元素
    let mut v = vec![1, 2, 3];
    for item in v.iter_mut() {
        *item *= 2; // item 是 &mut i32
    }
    
    // into_iter() - 获取所有权
    for item in v.into_iter() {
        println!("{}", item); // item 是 i32，v 被消费
    }
}
```

### 自定义迭代器

通过实现 `Iterator` 特质可以创建自定义迭代器：

```rust, editable
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);
    
    for num in counter {
        println!("{}", num); // 打印 1, 2, 3, 4, 5
    }
}
```

### 性能优势

Rust 的迭代器是零成本抽象。编译器会将迭代器链优化为高效机器码，通常和手写循环一样快：

```rust, editable
fn main() {
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // 迭代器方式（惯用且高效）
    let sum: i32 = numbers.iter().sum();
    
    // 手动循环（优化后性能相同）
    let mut sum2 = 0;
    for &num in &numbers {
        sum2 += num;
    }
}
```

### 惰性求值

迭代器在被消费前不会执行工作。这使得在不产生中间分配的情况下高效地链接操作成为可能：

```rust, editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 这会创建一个迭代器链，但尚未执行
    let iter = numbers.iter()
        .map(|x| {
            println!("Mapping {}", x);
            x * 2
        });
    
    // 只有在消费时才会执行映射
    let result: Vec<i32> = iter.collect();
}
```

### 最佳实践

- **优先使用迭代器而非手动索引**，可使代码更简洁且更安全
- **使用迭代器适配器** 以声明式表达转换
- **链接操作**，避免创建中间集合
- **谨慎使用 `collect()`**——仅在需要整个结果时使用
- **利用 `IntoIterator`**——实现该特质的类型可以与 `for` 循环一起使用
