### Modules in Rust

In Rust, modules are a way to organize code into separate namespaces. They help in managing scope and privacy, making the code more modular and easier to maintain.

### Defining a Module

You can define a module using the `mod` keyword. Here's a basic example:

```rust
mod my_module {
    pub fn say_hello() {
        println!("Hello from my_module!");
    }
}

fn main() {
    my_module::say_hello();
}
```

In this example:
- `mod my_module` defines a module named `my_module`.
- `pub fn say_hello()` defines a public function within the module.
- `my_module::say_hello()` calls the function from the main function.

### Nested Modules

Modules can be nested within other modules:

```rust
mod outer_module {
    pub mod inner_module {
        pub fn say_hello() {
            println!("Hello from inner_module!");
        }
    }
}

fn main() {
    outer_module::inner_module::say_hello();
}
```

In this example:
- `pub mod inner_module` defines a nested module within `outer_module`.
- `outer_module::inner_module::say_hello()` calls the function from the main function.

### Module Files

Modules can also be defined in separate files. For example, you can create a file structure like this:

```
src/
├── main.rs
└── my_module.rs
```

In `main.rs`:

```rust,noplaypen
mod my_module;

fn main() {
    my_module::say_hello();
}
```

In `my_module.rs`:

```rust,noplaypen
pub fn say_hello() {
    println!("Hello from my_module!");
}
```

This structure helps in keeping the codebase organized and manageable.
