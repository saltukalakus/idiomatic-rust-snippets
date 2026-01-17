### 向量（Vectors）

Rust 中的向量是标准库提供的动态数组类型，表示为 `Vec<T>`，其中 `T` 是向量中元素的类型。向量可以在运行时增长或缩小，并提供高效的索引和迭代操作。

### 创建向量

可以使用 `Vec::new` 或 `vec!` 宏创建向量：

```rust,ignore
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3, 4, 5];
```

### 添加元素

可以使用 `push` 方法向向量添加元素：

```rust,ignore
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### 访问元素

可以使用索引或 `get` 方法访问向量中的元素：

```rust, editable
let v = vec![1, 2, 3, 4, 5];
println!("The third element is {}", v[2]);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### 遍历向量

可以使用 `for` 循环遍历向量元素：

```rust, editable
let v = vec![1, 2, 3, 4, 5];
for i in &v {
    println!("{}", i);
}
```

### 删除元素

可以使用 `pop` 或 `remove` 方法从向量中删除元素：

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];
v.pop(); // Removes the last element
v.remove(1); // Removes the element at index 1
```

向量是 Rust 中用途广泛且强大的集合类型，适用于需要动态数组的多种场景。
