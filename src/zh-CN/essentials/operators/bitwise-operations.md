### 位运算符

位运算符用于对整数类型的各个位进行操作。

`&` : 按位与（Bitwise AND）<br/>
`|` : 按位或（Bitwise OR）<br/>
`^` : 按位异或（Bitwise XOR）<br/>
`<<` : 左移（Left shift）<br/>
`>>` : 右移（Right shift）<br/>

```rust, editable
fn main() {
    let a = 0b1100;
    let b = 0b1010;

    println!("a & b: {:04b}", a & b); // 按位与
    println!("a | b: {:04b}", a | b); // 按位或
    println!("a ^ b: {:04b}", a ^ b); // 按位异或
    println!("a << 1: {:04b}", a << 1); // 左移
    println!("a >> 1: {:04b}", a >> 1); // 右移
}
```
