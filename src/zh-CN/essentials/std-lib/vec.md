### Vec 类型与 `vec!` 宏

`Vec` 是可增长的数组类型，向量中的所有元素必须具有相同类型。`vec!` 宏用于创建向量。

当你需要在运行时动态增长或缩小集合时，`Vec` 非常有用。

```rust, editable
// 创建向量
let v1: Vec<i32> = Vec::new();     // 空向量
let v2 = vec![1, 2, 3];            // 带有初始值的向量
let v3 = vec!["aa", "dd", "cc"];   // 带有字符串类型项的向量

// 添加元素
let mut v4 = Vec::new();
v4.push(4);                        // [4]
v4.push(5);                        // [4, 5]


// 访问元素
let third = v2[2];                 // 3
let first = v2.get(0);             // Some(1)

// 向量长度
println!("长度: {}", v2.len());

// 向量内容
println!("向量 1: {:?}", v1);
println!("向量 2: {:?}", v2);
println!("向量 3: {:?}", v3);
println!("向量 4: {:?}", v4);
```
