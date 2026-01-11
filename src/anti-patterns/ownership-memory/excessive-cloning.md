### Excessive Cloning

Cloning data to satisfy the borrow checker is a common beginner anti-pattern. While `.clone()` can quickly resolve ownership errors, it creates unnecessary allocations and copies. This impacts performance, especially with large data structures or in tight loops.

Understanding Rust's borrowing rules allows you to share data efficiently without cloning. References are zero-cost abstractions.

```rust
use std::time::Instant;

fn analyze_text(text: String) -> (usize, usize) {
    let word_count = text.clone().split_whitespace().count();
    let char_count = text.clone().chars().count();
    let line_count = text.clone().lines().count();
    (word_count, line_count)
}

fn main() {
    let large_text = "word ".repeat(100_000);
    let start = Instant::now();
    
    let (words, lines) = analyze_text(large_text);
    let duration = start.elapsed();
    
    println!("Words: {}, Lines: {}", words, lines);
    println!("Time: {:?}", duration); // Much slower due to cloning
}
```

Cloning the string three times creates three separate 600KB allocations for a simple analysis. Each `.clone()` copies the entire string into a new heap allocation. For large texts or frequent calls, this wastes memory and CPU time. The anti-pattern artificially inflates memory usage by 4x (original + 3 clones).

The next sample uses borrowing instead of cloning.

```rust
use std::time::Instant;

fn analyze_text(text: &str) -> (usize, usize) {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    let line_count = text.lines().count();
    (word_count, line_count)
}

fn main() {
    let large_text = "word ".repeat(100_000);
    let start = Instant::now();
    
    let (words, lines) = analyze_text(&large_text);
    let duration = start.elapsed();
    
    println!("Words: {}, Lines: {}", words, lines);
    println!("Time: {:?}", duration); // Much faster - no cloning
}
```

**Key Improvements**:
- Accepts `&str` instead of `String` - works with borrowed data
- Zero allocations during analysis - only reads the original data
- Significantly faster - typically 3-5x faster for large texts
- Lower memory footprint - no duplicate allocations
- More flexible API - can accept `&String`, `&str`, or string literals
