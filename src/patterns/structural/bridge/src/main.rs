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