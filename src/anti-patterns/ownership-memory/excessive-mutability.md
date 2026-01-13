### Excessive Mutability

Marking variables as `mut` when they don't need to change is a common anti-pattern. Excessive mutability makes code harder to reason about, enables accidental modifications, and prevents compiler optimizations. Rust's default immutability is a feature, not a limitation.

Immutability communicates intent: this value won't change. It makes code more predictable and easier to debug.

```rust, editable
fn calculate_price(base_price: f64, quantity: u32) -> f64 {
    let mut price = base_price;
    let mut qty = quantity;
    let mut tax_rate = 0.08;
    
    let subtotal = price * qty as f64;
    tax_rate = 0.1; // Accidental modification - should have stayed 0.08
    
    subtotal * (1.0 + tax_rate)
}

fn main() {
    let total = calculate_price(10.0, 5);
    println!("Total: ${:.2}", total); // Prints: $55.00 (wrong, should be $54.00)
    
    let expected = 10.0 * 5.0 * 1.08;
    println!("Expected: ${:.2}", expected); // Prints: $54.00
}
```

All three variables are marked `mut` but only used once - none actually need mutation. The accidental reassignment of `tax_rate` from `0.08` to `0.1` goes unnoticed because the compiler allows it. This changes the tax from 8% to 10%, producing incorrect calculations. The bug is silent and easy to miss in code review.

The next sample uses immutability by default.

```rust, editable
fn calculate_price(base_price: f64, quantity: u32) -> f64 {
    let price = base_price;
    let qty = quantity;
    let tax_rate = 0.08;
    
    let subtotal = price * qty as f64;
    // tax_rate = 0.1; // Compiler error: cannot assign twice to immutable variable
    
    subtotal * (1.0 + tax_rate)
}

fn main() {
    let total = calculate_price(10.0, 5);
    println!("Total: ${:.2}", total); // Prints: $54.00 (correct)
    
    let expected = 10.0 * 5.0 * 1.08;
    println!("Expected: ${:.2}", expected); // Prints: $54.00
}
```

**Key Improvements**:
- Variables are immutable by default - accidental mutations prevented
- If the tax_rate reassignment is attempted, it won't compile
- Clearer intent - readers know these values don't change
- Easier to refactor - no need to track mutation points
- Enables compiler optimizations - immutable values can be optimized more aggressively
