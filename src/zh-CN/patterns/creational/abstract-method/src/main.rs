// 为产品定义 trait
trait Chair {
    fn has_legs(&self) -> bool;
    fn sit_on(&self);
}

trait Sofa {
    fn has_legs(&self) -> bool;
    fn lie_on(&self);
}

// 现代风格的具体产品
struct ModernChair;
struct ModernSofa;

impl Chair for ModernChair {
    fn has_legs(&self) -> bool {
        true
    }

    fn sit_on(&self) {
        println!("坐在现代风格的椅子上。");
    }
}

impl Sofa for ModernSofa {
    fn has_legs(&self) -> bool {
        true
    }

    fn lie_on(&self) {
        println!("躺在现代风格的沙发上。");
    }
}

// 维多利亚风格的具体产品
struct VictorianChair;
struct VictorianSofa;

impl Chair for VictorianChair {
    fn has_legs(&self) -> bool {
        true
    }

    fn sit_on(&self) {
        println!("坐在维多利亚风格的椅子上。");
    }
}

impl Sofa for VictorianSofa {
    fn has_legs(&self) -> bool {
        true
    }

    fn lie_on(&self) {
        println!("躺在维多利亚风格的沙发上。");
    }
}

// 抽象工厂 trait
trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_sofa(&self) -> Box<dyn Sofa>;
}

// 具体工厂
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

// 客户端 code
fn main() {
    let factory: Box<dyn FurnitureFactory> = Box::new(ModernFurnitureFactory);
    let chair = factory.create_chair();
    let sofa = factory.create_sofa();

    chair.sit_on();
    sofa.lie_on();

    let factory2: Box<dyn FurnitureFactory> = Box::new(VictorianFurnitureFactory);
    let chair2 = factory2.create_chair();
    let sofa2 = factory2.create_sofa();

    chair2.sit_on();
    sofa2.lie_on();
}