### Unnecessary Unsafe

Using `unsafe` blocks when safe Rust alternatives exist defeats the purpose of Rust's safety guarantees. Unsafe code requires manual verification that invariants are maintained. Developers often reach for `unsafe` prematurely to work around borrow checker errors or for perceived performance gains.

Unsafe should be a last resort. Most performance-critical code doesn't need it, and safe abstractions are usually fast enough.

```rust, editable
fn get_middle_element(data: &Vec<i32>) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    
    let mid = data.len() / 2;
    unsafe {
        // Unnecessary unsafe - could panic if logic is wrong
        Some(*data.get_unchecked(mid))
    }
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    if let Some(mid) = get_middle_element(&numbers) {
        println!("Middle: {}", mid); // Prints: Middle: 30
    }
    
    let empty: Vec<i32> = vec![];
    if let Some(mid) = get_middle_element(&empty) {
        println!("Middle: {}", mid);
    } else {
        println!("Empty vector"); // Prints: Empty vector
    }
}
```

Using `get_unchecked()` bypasses bounds checking, assuming the programmer verified the index is valid. If the `is_empty()` check is removed by mistake or the index calculation is wrong, undefined behavior occurs. This unsafe block provides no performance benefit since the bounds check is trivial, but it introduces potential memory unsafety.

The next sample uses safe indexing.

```rust, editable
fn get_middle_element(data: &Vec<i32>) -> Option<i32> {
    let mid = data.len() / 2;
    data.get(mid).copied()
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    if let Some(mid) = get_middle_element(&numbers) {
        println!("Middle: {}", mid); // Prints: Middle: 30
    }
    
    let empty: Vec<i32> = vec![];
    if let Some(mid) = get_middle_element(&empty) {
        println!("Empty vector"); // Prints: Empty vector
    } else {
        println!("No middle element");
    }
}
```

**Key Improvements**:
- Uses `.get()` which returns `Option<&T>` - cannot panic
- Automatically handles empty vectors by returning `None`
- No manual bounds checking required - the safe API handles it
- `.copied()` dereferences and copies the value safely
- Compiler can verify correctness - no unsafe invariants to maintain
