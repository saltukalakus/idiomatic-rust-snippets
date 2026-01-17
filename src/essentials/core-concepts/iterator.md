### Iterators

Iterators are one of Rust's most powerful features for working with sequences of elements. An iterator is any type that implements the `Iterator` trait, which allows you to process items in a sequence one at a time. Iterators are lazy, meaning they don't do any work until you consume them.

### The Iterator Trait

The core of Rust's iterator system is the `Iterator` trait, which requires implementing the `next` method:

```rust, editable
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Create an iterator
    let mut iter = numbers.iter();
    
    // Manually call next
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    
    // Use in a for loop (consumes the iterator)
    for num in numbers.iter() {
        println!("{}", num);
    }
}
```

### Iterator Adapters

Iterator adapters transform one iterator into another. They are lazy and don't consume the iterator until a consumer method is called.

```rust, editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Chain multiple adapters
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)           // Double each number
        .filter(|x| *x > 5)       // Keep only numbers > 5
        .collect();                // Consume and collect into Vec
    
    println!("{:?}", result); // [6, 8, 10]
}
```

### Common Iterator Methods

**Adapters (return new iterators)**:
- `map()`: Transform each element
- `filter()`: Keep elements matching a predicate
- `take()`: Take first n elements
- `skip()`: Skip first n elements
- `enumerate()`: Add indices to elements
- `zip()`: Combine two iterators
- `chain()`: Concatenate iterators
- `flat_map()`: Map and flatten nested structures

**Consumers (produce final values)**:
- `collect()`: Gather elements into a collection
- `sum()`, `product()`: Calculate sum or product
- `fold()`: Reduce to a single value
- `find()`: Find first matching element
- `any()`, `all()`: Check if any/all match predicate
- `count()`: Count elements
- `nth()`: Get nth element
- `for_each()`: Execute closure for each element

### Iterator Types

```rust, editable
fn main() {
    let v = vec![1, 2, 3];
    
    // iter() - borrows each element
    for item in v.iter() {
        println!("{}", item); // item is &i32
    }
    
    // iter_mut() - mutably borrows each element
    let mut v = vec![1, 2, 3];
    for item in v.iter_mut() {
        *item *= 2; // item is &mut i32
    }
    
    // into_iter() - takes ownership
    for item in v.into_iter() {
        println!("{}", item); // item is i32, v is consumed
    }
}
```

### Custom Iterators

You can create custom iterators by implementing the `Iterator` trait:

```rust, editable
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);
    
    for num in counter {
        println!("{}", num); // Prints 1, 2, 3, 4, 5
    }
}
```

### Performance Benefits

Iterators in Rust are zero-cost abstractions. The compiler optimizes iterator chains into efficient machine code, often as fast as hand-written loops:

```rust, editable
fn main() {
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Iterator approach (idiomatic and efficient)
    let sum: i32 = numbers.iter().sum();
    
    // Manual loop (same performance after optimization)
    let mut sum2 = 0;
    for &num in &numbers {
        sum2 += num;
    }
}
```

### Lazy Evaluation

Iterators don't perform work until consumed. This allows for efficient chaining without intermediate allocations:

```rust, editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // This creates an iterator chain but doesn't execute yet
    let iter = numbers.iter()
        .map(|x| {
            println!("Mapping {}", x);
            x * 2
        });
    
    // Only when consumed does the work happen
    let result: Vec<i32> = iter.collect();
}
```

### Best Practices

- **Prefer iterators over manual indexing** for cleaner, safer code
- **Use iterator adapters** to express transformations declaratively
- **Chain operations** instead of creating intermediate collections
- **Use `collect()` sparingly** - only when you need the entire result
- **Leverage `IntoIterator`** - types that implement it work with `for` loops
