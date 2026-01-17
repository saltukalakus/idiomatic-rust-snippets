// 抽象
trait Shape {
    fn draw(&self);
}

// 实现者
trait Color {
    fn fill(&self);
}

// 具体实现者 1
struct Red;

impl Color for Red {
    fn fill(&self) {
        println!("填充红色");
    }
}

// 具体实现者 2
struct Blue;

impl Color for Blue {
    fn fill(&self) {
        println!("填充蓝色");
    }
}

// 精化抽象
struct Circle {
    color: Box<dyn Color>,
}

impl Shape for Circle {
    fn draw(&self) {
        print!("Drawing Circle - ");
        self.color.fill();
    }
}

// 精化抽象
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