### Extension Traits

Extension traits allow you to add methods to types you don't own. This is a common pattern in Rust to extend functionality of external types (like those from `std` or other crates) without modifying them.

**Benefits**:
- Add methods to types from external crates
- Keep implementations organized and modular
- Work around Rust's orphan rule for traits
- Create domain-specific APIs on existing types

```rust
// Extension trait for String/&str
trait StringExt {
    fn is_blank(&self) -> bool;
    fn truncate_with_ellipsis(&self, max_len: usize) -> String;
}

impl StringExt for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
    
    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &self[..max_len - 3])
        }
    }
}

// Extension trait for iterators
trait IteratorExt: Iterator {
    fn sum_by<B, F>(self, f: F) -> B
    where
        Self: Sized,
        B: std::iter::Sum,
        F: FnMut(Self::Item) -> B;
}

impl<I: Iterator> IteratorExt for I {
    fn sum_by<B, F>(self, mut f: F) -> B
    where
        B: std::iter::Sum,
        F: FnMut(Self::Item) -> B,
    {
        self.map(|item| f(item)).sum()
    }
}

// Extension trait for Option
trait OptionExt<T> {
    fn ok_or_else_log<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        E: std::fmt::Display;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_else_log<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        E: std::fmt::Display,
    {
        match self {
            Some(v) => Ok(v),
            None => {
                let e = err();
                eprintln!("Warning: {}", e);
                Err(e)
            }
        }
    }
}

// Extension trait for Vec
trait VecExt<T> {
    fn push_if_not_exists(&mut self, value: T)
    where
        T: PartialEq;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_if_not_exists(&mut self, value: T)
    where
        T: PartialEq,
    {
        if !self.contains(&value) {
            self.push(value);
        }
    }
}

fn main() {
    // Using StringExt
    println!("=== StringExt ===");
    let text = "   ";
    println!("'{}' is blank: {}", text, text.is_blank());
    
    let long_text = "This is a very long string that needs truncation";
    println!("Truncated: {}", long_text.truncate_with_ellipsis(20));
    
    // Using IteratorExt
    println!("\n=== IteratorExt ===");
    struct Item { price: i32 }
    let items = vec![
        Item { price: 10 },
        Item { price: 20 },
        Item { price: 30 },
    ];
    let total: i32 = items.iter().sum_by(|item| item.price);
    println!("Total price: {}", total);
    
    // Using OptionExt
    println!("\n=== OptionExt ===");
    let value: Option<i32> = None;
    let result = value.ok_or_else_log(|| "Value was not found");
    println!("Result: {:?}", result);
    
    // Using VecExt
    println!("\n=== VecExt ===");
    let mut numbers = vec![1, 2, 3];
    numbers.push_if_not_exists(2); // Won't add - already exists
    numbers.push_if_not_exists(4); // Will add
    println!("Numbers: {:?}", numbers);
}
```

**Key Points**:
- Define a trait with the methods you want to add
- Implement the trait for the external type
- Users must import the trait to use the extension methods
- Convention: name the trait `{TypeName}Ext`
- Use `Self: Sized` bound for methods that consume self

**Common Extension Traits in Rust Ecosystem**:
- `itertools::Itertools` - extends Iterator with many methods
- `futures::StreamExt` - async stream extensions
- `anyhow::Context` - extends Result with context methods

**When to Use**:
- Adding utility methods to standard library types
- Creating domain-specific APIs for existing types
- Providing convenience methods in your library
- When you can't modify the original type
