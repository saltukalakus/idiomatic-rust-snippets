### Bitwise Operators

Bitwise operators are used to perform operations on individual bits of integer types.

`&` : Bitwise AND<br/>
`|` : Bitwise OR<br/>
`^` : Bitwise XOR<br/>
`<<` : Left shift<br/>
`>>` : Right shift<br/>

```rust, editable
fn main() {
    let a = 0b1100;
    let b = 0b1010;

    println!("a & b: {:04b}", a & b); // Bitwise AND
    println!("a | b: {:04b}", a | b); // Bitwise OR
    println!("a ^ b: {:04b}", a ^ b); // Bitwise XOR
    println!("a << 1: {:04b}", a << 1); // Left shift
    println!("a >> 1: {:04b}", a >> 1); // Right shift
}
```