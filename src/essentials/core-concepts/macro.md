### Macros in Rust

Macros in Rust are a way of writing code that writes other code, which is known as metaprogramming. They are used to reduce code repetition and to provide more expressive syntax.

### Types of Macros

1. **Declarative Macros (macro_rules!)**
2. **Procedural Macros**

### Declarative Macros

Declarative macros are defined using the `macro_rules!` keyword. They allow you to match against patterns and generate code based on those patterns.

```rust,ignore
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

Procedural macros are more flexible and powerful. They are defined using functions and can be used to create attribute-like macros and function-like macros.

#### Function-like Macros

Function-like macros look like function calls but operate on the code passed to them as arguments. They are defined using the `#[proc_macro]` attribute and can be used to generate code based on the input provided.

To create a procedural macro, you need to create a new crate with the `proc-macro` attribute:

```rust,ignore
// In your Cargo.toml
[lib]
proc-macro = true
```

Then, define the macro in your crate:

```rust,ignore
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_function_like_macro(input: TokenStream) -> TokenStream {
    // Your macro implementation here
    input
}
```

And use it in your main crate:

```rust,ignore
use my_function_like_macro::my_function_like_macro;

my_function_like_macro! {
    // Your code here
}
```

`my_function_like_macro` is a function-like macro that can be invoked with the `!` syntax. The macro processes the input token stream and generates the corresponding code.

#### Attribute-like Macros

Attribute-like macros are used to create custom attributes that can be applied to items such as functions, structs, or modules. They are defined similarly to procedural macros.

First, create a new crate with the `proc-macro` attribute:

```rust,ignore
// In your Cargo.toml
[lib]
proc-macro = true
```

Then, define the attribute-like macro in your crate:

```rust,ignore
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Your macro implementation here
    item
}
```

And use it in your main crate:

```rust,ignore
use my_attribute::my_attribute;

#[my_attribute]
fn my_function() {
    println!("This function has a custom attribute!");
}

fn main() {
    my_function();
}
```

`my_attribute` is an attribute-like macro that can be applied to a function. When `my_function` is called, it will print a message to the console.



### Usage

Macros are invoked using the `!` syntax. For example:

```rust,ignore
println!("Hello, {}!", "world");
```

`println!` is a macro that prints formatted text to the console.
