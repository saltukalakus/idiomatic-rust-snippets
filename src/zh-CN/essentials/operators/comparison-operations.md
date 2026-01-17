### 比较运算符

比较运算符用于比较两个值并返回布尔结果。

`==` : 等于<br/>
`!=` : 不等于<br/>
`>` : 大于<br/>
`<` : 小于<br/>
`>=` : 大于等于<br/>
`<=` : 小于等于<br/>

```rust, editable
fn main() {
    let a = 10;
    let b = 3;

    println!("a == b: {}", a == b); // Equal to
    println!("a != b: {}", a != b); // Not equal to
    println!("a > b: {}", a > b); // Greater than
    println!("a < b: {}", a < b); // Less than
    println!("a >= b: {}", a >= b); // Greater than or equal to
    println!("a <= b: {}", a <= b); // Less than or equal to
}
```
