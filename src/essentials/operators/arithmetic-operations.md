### Basic arithmetic operations

```rust
fn main() {
    let x = 5;
    let y = 6;

    // Addition
    let sum = x + y;
    println!("Sum: {}", sum);

    // Subtraction
    let difference = x - y;
    println!("Difference: {}", difference);

    // Multiplication
    let product = x * y;
    println!("Product: {}", product);

    // Division
    let quotient = x / y; // Note: This will perform integer division
    println!("Quotient: {}", quotient);

    // Remainder (Modulus)
    let remainder = x % y;
    println!("Remainder: {}", remainder);

    // Assigning the result to an immutable variable 
    let z = x + y;
    // z += 1; // Will return a compilation failure as by default z is immutable

    println!("Immutable z: {}", z);

    // Mutable variable
    let mut zz = x + y;
    zz += 1;

    println!("Mutable zz: {}", zz);
}
```
