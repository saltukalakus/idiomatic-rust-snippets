### What is **trait**?

In Rust, a trait is a way to define shared behavior in an abstract way. It is similar to interfaces in other programming languages. A trait defines a set of methods that a type must implement. Traits are used to define shared behavior and to enable polymorphism.

### Purpose of Traits

**Define Shared Behavior**: Traits allow you to define methods that can be shared across different types.<br/>
**Enable Polymorphism**: Traits enable polymorphism, allowing you to write code that can operate on different types in a generic way.<br/>
**Abstract Over Types**: Traits allow you to abstract over types, enabling you to write more flexible and reusable code.<br/>

In the sample code below the Greet trait is defined with a single method greet. The Greet trait is implemented for two structs, Person and Dog. 


```rust, editable
// Define a trait named `Greet`
pub trait Greet {
    fn greet(&self);
}

// Define a struct named `Person`
pub struct Person {
    name: String,
}

// Implement the `Greet` trait for the `Person` struct
impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

// Define a struct named `Dog`
pub struct Dog {
    name: String,
}

// Implement the `Greet` trait for the `Dog` struct
impl Greet for Dog {
    fn greet(&self) {
        println!("Woof! My name is {}!", self.name);
    }
}


fn main() {
    // Create instances of `Person` and `Dog`
    let person = Person {
        name: String::from("Alice"),
    };
    let dog = Dog {
        name: String::from("Buddy"),
    };
    dog.greet();
    person.greet();
}
```
