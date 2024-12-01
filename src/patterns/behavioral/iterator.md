### Iterator Pattern

The iterator pattern is a design pattern that provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation. In Rust, the iterator pattern is implemented using the `Iterator` trait.

Here is an example of how to implement and use the iterator pattern by defining a simple iterator that iterates over a range of numbers:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    while let Some(value) = counter.next() {
        println!("{}", value);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("The sum is: {}", sum);
}
```

### Using Iterator Methods

Rust's standard library provides many useful methods for iterators. In this example, we use the `zip`, `skip`, `map`, `filter`, and `sum` methods to perform various operations on the iterator.

The iterator pattern in Rust is a powerful and flexible way to work with sequences of data. By implementing the `Iterator` trait, you can create custom iterators and take advantage of the many methods provided by the standard library to manipulate and process data efficiently.