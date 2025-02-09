### Lifetime in Rust

In Rust, lifetimes are a way of ensuring that references are valid for as long as they are needed and no longer. Lifetimes prevent dangling references, which can lead to undefined behavior.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

The function `longest` takes two string slices with the same lifetime `'a` and returns a string slice with the same lifetime `'a`. This ensures that the returned reference is valid as long as both input references are valid.

### Key Points

- Lifetimes are denoted with an apostrophe followed by a name, like `'a`.
- They help the Rust compiler ensure that references do not outlive the data they point to.
- Lifetimes are inferred by the compiler in many cases, but sometimes they need to be explicitly annotated.

For more detailed information, refer to the [Rust Book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).