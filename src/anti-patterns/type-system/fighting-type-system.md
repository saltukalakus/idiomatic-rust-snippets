### Fighting the Type System

Using type casts, transmutes, or `as` conversions to force types to fit instead of using proper types leads to bugs. Rust's type system prevents many errors at compile time. Fighting it with unsafe casts or ignoring type mismatches creates runtime problems that types were designed to prevent.

When the type system resists, it's usually highlighting a design issue. Work with the types, not against them.

```rust, editable
fn calculate_average(numbers: Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    sum / count // Can panic with division by zero
}

fn process_bytes(data: &[u8]) -> u32 {
    // Dangerous: assumes data has at least 4 bytes
    unsafe {
        let ptr = data.as_ptr() as *const u32;
        *ptr // Undefined behavior if data.len() < 4
    }
}

fn main() {
    let numbers = vec![10, 20, 30];
    let avg = calculate_average(numbers);
    println!("Average: {}", avg); // Prints: Average: 20
    
    let empty: Vec<i32> = vec![];
    // This panics: thread 'main' panicked at 'attempt to divide by zero'
    let avg = calculate_average(empty);
    println!("Average: {}", avg);
}
```

`calculate_average` panics on empty vectors - division by zero. The function signature promises it always returns `i32`, but it can't. Using `as i32` casts work around type issues instead of fixing them. `process_bytes` uses unsafe to transmute bytes to `u32`, causing undefined behavior if fewer than 4 bytes exist.

The next sample uses appropriate types.

```rust, editable
fn calculate_average(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;
    Some(sum as f64 / count)
}

fn process_bytes(data: &[u8]) -> Option<u32> {
    if data.len() < 4 {
        return None;
    }
    
    let bytes: [u8; 4] = [data[0], data[1], data[2], data[3]];
    Some(u32::from_le_bytes(bytes))
}

fn main() {
    let numbers = vec![10, 20, 30];
    if let Some(avg) = calculate_average(&numbers) {
        println!("Average: {:.2}", avg); // Prints: Average: 20.00
    }
    
    let empty: Vec<i32> = vec![];
    match calculate_average(&empty) {
        Some(avg) => println!("Average: {:.2}", avg),
        None => println!("Cannot calculate average of empty list"), // Prints this
    }
}
```

**Key Improvements**:
- Returns `Option<f64>` - type signature reflects that average may not exist
- Uses `f64` for average - integer division loses precision
- Cannot panic - all error cases return `None`
- `process_bytes` safely checks length before conversion
- Uses `u32::from_le_bytes()` instead of unsafe transmute
