### Flow Control

Rust provides several options for flow control, including `if` statements, `while` loops, and more. Here are some simple examples for each:

### `if` Statements

The `if` statement allows you to execute code based on a condition.

```rust, editable
let number = 5;

if number < 10 {
    println!("The number is less than 10");
} else {
    println!("The number is 10 or greater");
}
```

### `while` Loops

The `while` loop allows you to execute code repeatedly as long as a condition is true.

```rust, editable
let mut count = 0;

while count < 5 {
    println!("Count is: {}", count);
    count += 1;
}
```

### `for` Loops

The `for` loop allows you to iterate over a range or collection.

```rust, editable
for number in 1..5 {
    println!("The number is: {}", number);
}
```

### `match` Statements

The `match` statement allows you to compare a value against a series of patterns and execute code based on which pattern matches.

```rust, editable
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),
}
```

These are some of the basic flow control options available in Rust. Each of these constructs helps you manage the flow of your program effectively.