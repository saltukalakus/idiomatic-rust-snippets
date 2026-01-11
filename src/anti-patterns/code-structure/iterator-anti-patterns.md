### Not Using Iterators

Collecting intermediate results when iterators can chain operations directly is inefficient and verbose. Creating temporary vectors between operations wastes memory and obscures the data flow. This anti-pattern often appears when developers are unfamiliar with iterator adapters.

Iterator chains are lazy, meaning they only process elements as needed, and the compiler can optimize the entire chain as a single operation.

```rust
fn process_numbers(numbers: Vec<i32>) -> Vec<String> {
    // Step 1: Filter
    let mut filtered = Vec::new();
    for n in &numbers {
        if *n > 0 {
            filtered.push(*n);
        }
    }
    
    // Step 2: Square
    let mut squared = Vec::new();
    for n in &filtered {
        squared.push(n * n);
    }
    
    // Step 3: Convert to strings
    let mut result = Vec::new();
    for n in &squared {
        result.push(format!("#{}", n));
    }
    
    result
}

fn main() {
    let numbers = vec![-2, -1, 0, 1, 2, 3];
    let result = process_numbers(numbers);
    println!("{:?}", result); // Prints: ["#1", "#4", "#9"]
}
```

This code creates three temporary vectors (`filtered`, `squared`, `result`), each requiring separate allocations. Each loop traverses the data independently, resulting in three separate passes over the collection. This is inefficient in both time and space, and the logic is scattered across multiple loops.

The next sample uses iterator chaining.

```rust
fn process_numbers(numbers: Vec<i32>) -> Vec<String> {
    numbers.into_iter()
        .filter(|&n| n > 0)
        .map(|n| n * n)
        .map(|n| format!("#{}", n))
        .collect()
}

fn main() {
    let numbers = vec![-2, -1, 0, 1, 2, 3];
    let result = process_numbers(numbers);
    println!("{:?}", result); // Prints: ["#1", "#4", "#9"]
}
```

**Key Improvements**:
- Single pass through the data - no intermediate collections
- Each element flows through the entire pipeline before the next is processed
- More memory efficient - only the final result is allocated
- The data transformation pipeline is clear and declarative
- The compiler can optimize the entire chain as one operation
