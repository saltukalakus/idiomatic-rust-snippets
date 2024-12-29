// Define traits for products
trait Chair {
    fn has_legs(&self) -> bool;
    fn sit_on(&self);
}

trait Sofa {
    fn has_legs(&self) -> bool;
    fn lie_on(&self);
}

// Concrete products for Modern style
struct ModernChair;
struct ModernSofa;

impl Chair for ModernChair {
    fn has_legs(&self) -> bool {
        true
    }

    fn sit_on(&self) {
        println!("Sitting on a modern chair.");
    }
}

impl Sofa for ModernSofa {
    fn has_legs(&self) -> bool {
        true
    }

    fn lie_on(&self) {
        println!("Lying on a modern sofa.");
    }
}

// Concrete products for Victorian style
struct VictorianChair;
struct VictorianSofa;

impl Chair for VictorianChair {
    fn has_legs(&self) -> bool {
        true
    }

    fn sit_on(&self) {
        println!("Sitting on a Victorian chair.");
    }
}

impl Sofa for VictorianSofa {
    fn has_legs(&self) -> bool {
        true
    }

    fn lie_on(&self) {
        println!("Lying on a Victorian sofa.");
    }
}

// Abstract factory trait
trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_sofa(&self) -> Box<dyn Sofa>;
}

// Concrete factories
struct ModernFurnitureFactory;

impl FurnitureFactory for ModernFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair)
    }

    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(ModernSofa)
    }
}

struct VictorianFurnitureFactory;

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair)
    }

    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(VictorianSofa)
    }
}

// Client code
fn main() {
    let factory: Box<dyn FurnitureFactory> = Box::new(ModernFurnitureFactory);
    let chair = factory.create_chair();
    let sofa = factory.create_sofa();

    chair.sit_on();
    sofa.lie_on();
}