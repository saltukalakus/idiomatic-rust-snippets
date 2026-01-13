### Pattern Matching

Pattern matching is a powerful feature that allows you to match complex data structures and perform different actions based on their shape. 

### Matching Literals

```rust, editable
let x = 1;

match x {
    1 => println!("One!"),
    2 => println!("Two!"),
    _ => println!("Something else!"),
}
```

### Matching Multiple Patterns

```rust, editable
let x = 1;

match x {
    1 | 2 => println!("One or Two!"),
    3 => println!("Three!"),
    _ => println!("Something else!"),
}
```

### Destructuring Structs

```rust, editable
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

### Destructuring Enums

```rust, editable
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => println!("The Quit variant has no data to destructure."),
    Message::Move { x, y } => println!("Move in the x direction {} and in the y direction {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
}
```

### Ignoring Values in a Pattern

```rust, editable
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

### Matching Ranges of Values

```rust, editable
let x = 5;

match x {
    1..=5 => println!("One through Five"),
    _ => println!("Something else"),
}
```

### Using `if` Guards

```rust, editable
let x = Some(4);

match x {
    Some(n) if n < 5 => println!("Less than five: {}", n),
    Some(n) => println!("{}", n),
    None => (),
}
```