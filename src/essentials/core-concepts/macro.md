### Macros in Rust

Macros in Rust are a way of writing code that writes other code, which is known as metaprogramming. They are used to reduce code repetition and to provide more expressive syntax.

### Types of Macros

1. **Declarative Macros (macro_rules!)**
2. **Procedural Macros**

### Declarative Macros

Declarative macros are defined using the `macro_rules!` keyword. They allow you to match against patterns and generate code based on those patterns.

#### Example

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();
}
```

### Procedural Macros

Procedural macros are more flexible and powerful. They are defined using functions and can be used to create custom derive macros, attribute-like macros, and function-like macros.

#### Example

To create a procedural macro, you need to create a new crate with the `proc-macro` attribute:

```rust
// In your Cargo.toml
[lib]
proc-macro = true
```

Then, define the macro in your crate:

```rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Your macro implementation here
    input
}
```

And use it in your main crate:

```rust
use my_macro::my_macro;

my_macro! {
    // Your code here
}
```

### Usage

Macros are invoked using the `!` syntax. For example:

```rust
println!("Hello, {}!", "world");
```

In this example, `println!` is a macro that prints formatted text to the console.
