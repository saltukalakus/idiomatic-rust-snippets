// Define the Shape trait
trait Shape {
    fn draw(&self);
}

// Implement the Shape trait for Circle
struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

// Implement the Shape trait for Square
struct Square;

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

// Define the ShapeFactory trait
trait ShapeFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
}

// Implement the ShapeFactory trait for CircleFactory
struct CircleFactory;

impl ShapeFactory for CircleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle)
    }
}

// Implement the ShapeFactory trait for SquareFactory
struct SquareFactory;

impl ShapeFactory for SquareFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Square)
    }
}

fn main() {
    // Create a Circle using CircleFactory
    let circle_factory = CircleFactory;
    let circle = circle_factory.create_shape();
    circle.draw();

    // Create a Square using SquareFactory
    let square_factory = SquareFactory;
    let square = square_factory.create_shape();
    square.draw();
}