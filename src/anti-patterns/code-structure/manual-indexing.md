### Manual Indexing

Rust provides powerful iterator methods that make code more expressive, safer, and often more efficient than manual index-based loops. Using manual indexing with `for i in 0..length` is verbose, error-prone (potential panic on out-of-bounds access), and misses optimization opportunities that iterators provide.

Iterators also compose better, allowing you to chain operations like `.filter()`, `.map()`, and `.collect()` in a clear, functional style.

```rust, editable
fn sum_adjacent_pairs(numbers: Vec<i32>) -> Vec<i32> {
    let mut sums = Vec::new();
    
    for i in 0..numbers.len() {
        let sum = numbers[i] + numbers[i + 1]; // Panics when i = numbers.len() - 1
        sums.push(sum);
    }
    
    sums
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sums = sum_adjacent_pairs(numbers);
    println!("Adjacent sums: {:?}", sums);
}
```

Manual indexing with `numbers[i + 1]` causes a runtime panic. When `i` reaches the last index, `i + 1` is out of bounds. This is a common bug with manual indexing - off-by-one errors that the compiler cannot catch. The program crashes with: `thread 'main' panicked at 'index out of bounds'`.

The next sample uses idiomatic iterator methods to avoid this issue.

```rust, editable
fn sum_adjacent_pairs(numbers: Vec<i32>) -> Vec<i32> {
    numbers.windows(2)
        .map(|pair| pair[0] + pair[1])
        .collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sums = sum_adjacent_pairs(numbers);
    println!("Adjacent sums: {:?}", sums); // Prints: Adjacent sums: [3, 5, 7, 9]
}
```

**Key Improvements**:
- The `.windows(2)` method safely provides sliding windows of 2 elements
- Cannot panic - the iterator automatically handles bounds correctly
- More concise and expressive - clearly shows we're working with adjacent pairs
- The logic is correct by construction - no off-by-one errors possible
- Composable - can chain additional operations without index management
