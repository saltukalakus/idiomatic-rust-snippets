### 复合类型

复合类型可以将多个值组合成一个类型。Rust 有两种主要的复合类型：

元组（Tuples）：将不同类型的多个值组合在一起。<br/>
数组（Arrays）：将相同类型的多个值组合在一起。<br/>

示例：

```rust, editable
fn main() {
    // 元组
    let tuple: (i32, f64, char) = (42, 3.14, 'A');
    let (x, y, z) = tuple;
    println!("元组: ({}, {}, {})", x, y, z);

    // 数组
    let array: [i32; 3] = [1, 2, 3];
    println!("数组: {:?}", array);
}
```
