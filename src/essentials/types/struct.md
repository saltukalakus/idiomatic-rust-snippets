### What is a struct?

Structs in Rust are custom data types that allow you to group related data together. They are similar to classes in object-oriented languages and can have associated methods defined through `impl` blocks. Structs are used to create complex data types that can be used to model real-world entities.

### Types of Structs

**Classic Structs**: 

These are the most common type of structs, where each field has a name and a type. Classic structs have named fields. E.g.:

```rust, editable
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}
```

**Tuple Structs**: 

These are similar to tuples but have a name and can be used to create new types. Tuple structs have unnamed fields. E.g.:

```rust, editable
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);

    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
}
```

**Unit-like Structs**: 

These are structs without any fields and are useful for generics or traits. Unit-like structs have no fields. E.g.:

```rust, editable
struct Unit;

fn main() {
    let unit = Unit;

    println!("Unit struct created!");
}
```

You can define methods and associated functions for structs using the [impl](../std-lib/imp.md) keyword.