### 赋值运算符

赋值运算符用于将值赋给变量。Rust 还支持将算术操作与赋值组合的复合赋值运算符。

`=` : 赋值<br/>
`+=` : 加后赋值<br/>
`-=` : 减后赋值<br/>
`*=` : 乘后赋值<br/>
`/=` : 除后赋值<br/>
`%=` : 取模后赋值<br/>

```rust, editable
fn main() {
    let mut a = 10;

    a += 5; // Addition assignment
    println!("a += 5: {}", a);

    a -= 3; // Subtraction assignment
    println!("a -= 3: {}", a);

    a *= 2; // Multiplication assignment
    println!("a *= 2: {}", a);

    a /= 4; // Division assignment
    println!("a /= 4: {}", a);

    a %= 3; // Remainder assignment
    println!("a %= 3: {}", a);
}
```
