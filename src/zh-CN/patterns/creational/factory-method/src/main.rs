// 定义 Shape trait
trait Shape {
    fn draw(&self);
}

// 为 Circle 实现 Shape trait
struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("绘制一个圆形");
    }
}

// 为 Square 实现 Shape trait
struct Square;

impl Shape for Square {
    fn draw(&self) {
        println!("绘制一个正方形");
    }
}

// 定义 ShapeFactory trait
trait ShapeFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
}

// 为 CircleFactory 实现 ShapeFactory trait
struct CircleFactory;

impl ShapeFactory for CircleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle)
    }
}

// 为 SquareFactory 实现 ShapeFactory trait
struct SquareFactory;

impl ShapeFactory for SquareFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Square)
    }
}

fn main() {
    // 使用 CircleFactory 创建一个 Circle
    let circle_factory = CircleFactory;
    let circle = circle_factory.create_shape();
    circle.draw();

    // 使用 SquareFactory 创建一个 Square
    let square_factory = SquareFactory;
    let square = square_factory.create_shape();
    square.draw();
}