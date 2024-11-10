### Compound Types

Compound types can group multiple values into one type. Rust has two primary compound types:

Tuples: Group multiple values of different types.<br/>
Arrays: Group multiple values of the same type.<br/>

Example:

```rust
fn main() {
    // Tuple
    let tuple: (i32, f64, char) = (42, 3.14, 'A');
    let (x, y, z) = tuple;
    println!("Tuple: ({}, {}, {})", x, y, z);

    // Array
    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
}
```
