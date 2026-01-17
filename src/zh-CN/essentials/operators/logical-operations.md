### 逻辑运算符

逻辑运算符用于组合多个布尔表达式。

`&&` : 逻辑与（AND）<br/>
`||` : 逻辑或（OR）<br/>
`!` : 逻辑非（NOT）<br/>

```rust, editable
fn main() {
    let a = true;
    let b = false;

    println!("a && b: {}", a && b); // Logical AND
    println!("a || b: {}", a || b); // Logical OR
    println!("!a: {}", !a); // Logical NOT
}
```
