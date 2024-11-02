### Logical Operators

Logical operators are used to combine multiple boolean expressions.

`&&` : Logical AND<br/>
`||` : Logical OR<br/>
`!` : Logical NOT<br/>

```rust
fn main() {
    let a = true;
    let b = false;

    println!("a && b: {}", a && b); // Logical AND
    println!("a || b: {}", a || b); // Logical OR
    println!("!a: {}", !a); // Logical NOT
}
```