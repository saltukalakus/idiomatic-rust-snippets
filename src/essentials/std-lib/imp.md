### What is the purpose of **imp** keyword?

The **impl** keyword in Rust is used to define implementations for structs, enums, and [traits](../types/trait.md). It allows you to associate functions and methods with a type, and to implement traits for a type.

### Usage of impl

**Implementing Methods for a Struct or Enum**: 

You can define methods that are associated with a struct or enum.

`pub fn new(name: String, age: u8) -> Self` is an associated function (often used as a constructor) that creates a new instance of Person. `pub fn greet(&self)` and `pub fn have_birthday(&mut self)` are methods associated with the Person struct. The `greet` method borrows the instance immutably, while the `have_birthday` method borrows it mutably.

```rust, editable
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    // Associated function (constructor)
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    // Method that borrows the instance immutably
    pub fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }

    // Method that borrows the instance mutably
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person::new(String::from("Alice"), 30);
    person.greet();
    person.have_birthday();
    person.greet();
}
```

**Implementing Traits for a Struct or Enum**: 

You can implement traits for a struct or enum, defining the behavior required by the trait.

The `Greet` trait defines a method `greet`. The `impl Greet for Dog` block implements the Greet trait for the `Dog` struct, providing the behavior required by the trait.

```rust, editable
pub trait Greet {
    fn greet(&self);
}

pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
}

impl Greet for Dog {
    fn greet(&self) {
        println!("Woof! My name is {}!", self.name);
    }
}

fn main() {
    let dog = Dog::new(String::from("Buddy"));
    dog.greet();
}
```