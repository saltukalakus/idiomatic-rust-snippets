### Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:

**Integers**: Whole numbers, both signed and unsigned.<br/>
**Floating-Point Numbers**: Numbers with decimal points.<br/>
**Booleans**: True or false values.<br/>
**Characters**: Single Unicode characters.<br/>

```rust, editable
fn main() {
    let x: i32 = 42; // Integer
    let y: f64 = 3.14; // Floating-point number
    let is_active: bool = true; // Boolean
    let letter: char = 'A'; // Character

    println!("x: {}, y: {}, is_active: {}, letter: {}", x, y, is_active, letter);
}
```

