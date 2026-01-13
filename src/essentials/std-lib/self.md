### Is **self** a special keywork in Rust?

**self** is a special keyword in Rust. It is used to refer to the instance of the struct or enum within its associated methods. It is similar to this in other object-oriented languages like Java or C++.

### Usage of self

**Method Definitions**: When defining methods for a struct or enum, self is used to refer to the instance on which the method is called.<br/>
**Method Parameters**: self can be used as a parameter in method definitions to indicate that the method takes ownership, borrows immutably, or borrows mutably from the instance.<br/>

### Immutable Borrow (&self)

When a method takes &self as a parameter, it means the method borrows the instance immutably.

```rust, editable
pub struct Person {
    name: String,
}

impl Person {
    // Method that borrows the instance immutably
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet(); // Calls the greet method
}
```

### Mutable Borrow (&mut self)

When a method takes &mut self as a parameter, it means the method borrows the instance mutably.

```rust, editable
pub struct Counter {
    count: i32,
}

impl Counter {
    // Method that borrows the instance mutably
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment(); // Calls the increment method
    println!("Count: {}", counter.count);
    counter.increment(); // Calls the increment method
    println!("Count: {}", counter.count);
}
```

### Ownership (self)

When a method takes self as a parameter, it means the method takes ownership of the instance.

```rust, editable
pub struct Person {
    name: String,
}

impl Person {
    // Method that takes ownership of the instance
    fn into_name(self) -> String {
        self.name
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    let name = person.into_name(); // Calls the into_name method, taking ownership
    println!("Name: {}", name);
}
```