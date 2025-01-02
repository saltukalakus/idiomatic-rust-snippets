### Bridge Design Pattern

The Bridge pattern is used to separate the abstraction from its implementation, allowing them to vary independently. It is useful when both the class and what it does vary often.

Here is an example of the Bridge Design Pattern:

```rust
// Abstraction
trait Shape {
    fn draw(&self);
}

// Implementor
trait Color {
    fn fill(&self);
}

// Concrete Implementor 1
struct Red;

impl Color for Red {
    fn fill(&self) {
        println!("Filling with red color");
    }
}

// Concrete Implementor 2
struct Blue;

impl Color for Blue {
    fn fill(&self) {
        println!("Filling with blue color");
    }
}

// Refined Abstraction
struct Circle {
    color: Box<dyn Color>,
}

impl Shape for Circle {
    fn draw(&self) {
        print!("Drawing Circle - ");
        self.color.fill();
    }
}

// Refined Abstraction
struct Square {
    color: Box<dyn Color>,
}

impl Shape for Square {
    fn draw(&self) {
        print!("Drawing Square - ");
        self.color.fill();
    }
}

fn main() {
    let red = Box::new(Red);
    let blue = Box::new(Blue);

    let red_circle = Circle { color: red };
    let blue_square = Square { color: blue };

    red_circle.draw();
    blue_square.draw();
}
```

1. **Shape Trait**: Defines the abstraction with a `draw` method.
2. **Color Trait**: Defines the implementor with a `fill` method.
3. **Red and Blue Structs**: Concrete implementors of the `Color` trait.
4. **Circle and Square Structs**: Refined abstractions that implement the `Shape` trait and use a `Color` to fill the shape.
5. **main() Function**: Demonstrates the usage of the bridge pattern by creating shapes with different colors and drawing them.

This example shows how the Bridge pattern allows the abstraction (Shape) and the implementation (Color) to vary independently.