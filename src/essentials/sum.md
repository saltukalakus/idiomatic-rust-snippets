# Basic math operations

```rust
fn main() {
    let x = 5;
    let y = 6;

    // Assigning the result to an immutable variable 
    let z = x + y;
    // z += 1; // Will return a compilation failure as by default z is immutable

    println!("{}", z);

    // Mutable variable
    let mut zz = x + y;
    zz += 1;

    println!("{}", zz);
}