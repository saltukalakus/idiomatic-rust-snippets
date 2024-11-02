# Logical Operators

Logical operators are used to combine multiple boolean expressions.

&& : Logical AND
|| : Logical OR
! : Logical NOT

```rust
fn main() {
    let a = true;
    let b = false;

    println!("a && b: {}", a && b); // Logical AND
    println!("a || b: {}", a || b); // Logical OR
    println!("!a: {}", !a); // Logical NOT
}
```