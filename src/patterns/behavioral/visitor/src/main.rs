// Define the Visitor trait
trait Visitor {
    fn visit_element_a(&mut self, element: &ElementA);
    fn visit_element_b(&mut self, element: &ElementB);
}

// Define the Element trait
trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
}

// Define concrete elements
struct ElementA;
struct ElementB;

impl Element for ElementA {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_element_a(self);
    }
}

impl Element for ElementB {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_element_b(self);
    }
}

// Define a concrete visitor
struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_element_a(&mut self, element: &ElementA) {
        println!("Visiting ElementA");
    }

    fn visit_element_b(&mut self, element: &ElementB) {
        println!("Visiting ElementB");
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> = vec![Box::new(ElementA), Box::new(ElementB)];
    let mut visitor = ConcreteVisitor;

    for element in elements {
        element.accept(&mut visitor);
    }
}