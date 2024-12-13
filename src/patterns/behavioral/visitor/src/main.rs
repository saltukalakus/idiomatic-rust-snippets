// Define the Visitor trait
trait Visitor {
    fn multiply_by_two(&mut self, element: &mut ElementA);
    fn add_ten(&mut self, element: &mut ElementB);
}

// Define the Element trait
trait Element {
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

// Define concrete elements
struct ElementA {
    data: i32,
}

struct ElementB {
    data: i32,
}

impl Element for ElementA {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.multiply_by_two(self);
        println!("After the visit: {:?}", self.data);
    }
}

impl Element for ElementB {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.add_ten(self);
        println!("After the visit: {:?}", self.data);
    }
}

// Define a concrete visitor
struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn multiply_by_two(&mut self, element: &mut ElementA) {
        println!("Visiting ElementA: {:?}", element.data);
        element.data *= 2;
    }

    fn add_ten(&mut self, element: &mut ElementB) {
        println!("Visiting ElementB: {:?}", element.data);
        element.data += 10;
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> = vec![Box::new(ElementA { data: 10 }), Box::new(ElementB { data: 5 })];
    let mut visitor = ConcreteVisitor;

    for mut element in elements {
        element.accept(&mut visitor);
    }
}