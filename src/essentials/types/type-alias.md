# What is type alias in Rust?

In Rust, a type alias allows you to create a new name for an existing type. This can make your code more readable and easier to manage, especially when dealing with complex types. Type aliases are created using the `type` keyword.

### Benefits of Type Aliases

Improved Readability: By giving a meaningful name to a type, you can make your code more understandable. <br/>
Simplified Code: Type aliases can simplify complex type signatures, making the code easier to write and read.<br/>
Consistency: Using type aliases can help maintain consistency across your codebase.<br/>

```rust, editable
// type NewName = ExistingType;

type Kilometers = i32;

fn main() {
    let distance: Kilometers = 100;
    println!("Distance: {} km", distance);
}
```

`Kilometers` is a type alias for `i32`. This makes it clear that the distance variable represents a distance in kilometers.

```rust, editable
struct Point<T> {
    x: T,
    y: T,
}

type IntPoint = Point<i32>;
type FloatPoint = Point<f64>;

fn main() {
    let int_point: IntPoint = Point { x: 5, y: 10 };
    let float_point: FloatPoint = Point { x: 1.0, y: 4.0 };

    println!("IntPoint: ({}, {})", int_point.x, int_point.y);
    println!("FloatPoint: ({}, {})", float_point.x, float_point.y);
}
```
`IntPoint` and `FloatPoint` are type aliases for `Point<i32>` and `Point<f64>`, respectively. This makes it clear what type of points are being used.

Type aliases are particularly useful for complex types, such as function pointers or nested types. 

```rust, editable
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // Execute the function
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("Hello, world!"))
}

fn main() {
    let f: Thunk = returns_long_type();
    takes_long_type(f);
}
```

`type Thunk = Box<dyn Fn() + Send + 'static>;` creates a type alias named Thunk.<br/>
**Box**: Box is a smart pointer that allocates data on the heap.<br/>
**dyn Fn()**: dyn Fn() is a trait object representing a closure that takes no arguments and returns nothing.<br/>
**Send**: The Send trait indicates that the closure can be transferred across thread boundaries.<br/>
**'static**: The 'static lifetime means that the closure can live for the entire duration of the program.<br/>

