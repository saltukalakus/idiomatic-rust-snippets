### Assignment Operators

Assignment operators are used to assign values to variables. Rust also supports compound assignment operators that combine an arithmetic operation with assignment.

`=` : Assignment<br/>
`+=` : Addition assignment<br/>
`-=` : Subtraction assignment<br/>
`*=` : Multiplication assignment<br/>
`/=` : Division assignment<br/>
`%=` : Remainder assignment<br/>

```rust
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