### Comparison Operators

Comparison operators are used to compare two values and return a boolean result.

`==` : Equal to<br/>
`!=` : Not equal to<br/>
`>` : Greater than<br/>
`<` : Less than<br/>
`>=` : Greater than or equal to<br/>
`<=` : Less than or equal to<br/>

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