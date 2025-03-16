### HashMap

`HashMap` is a collection that stores key-value pairs where you can look up values by their associated key.

```rust
use std::collections::HashMap;

fn main() {
    // Create a new HashMap
    let mut scores = HashMap::new();

    // Insert some key-value pairs
    scores.insert("Blue", 10);
    scores.insert("Red", 50);

    // Get a value
    match scores.get("Blue") {
        Some(score) => println!("Blue team's score: {}", score),
        None => println!("Blue team not found"),
    }

    // Update a value
    scores.insert("Blue", 25); // This will overwrite the previous value

    // Insert only if key doesn't exist
    scores.entry("Yellow").or_insert(5);

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

Common operations:
- `new()`: Creates an empty HashMap
- `insert()`: Adds a key-value pair
- `get()`: Retrieves a value by key
- `entry()`: Gets entry for in-place manipulation
- `remove()`: Removes a key-value pairuse std::collections::HashMap