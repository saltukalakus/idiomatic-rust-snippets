struct SubsystemA;

impl SubsystemA {
    fn operation_a1(&self) -> String {
        "Subsystem A, Operation A1".to_string()
    }

    fn operation_a2(&self) -> String {
        "Subsystem A, Operation A2".to_string()
    }
}

struct SubsystemB;

impl SubsystemB {
    fn operation_b1(&self) -> String {
        "Subsystem B, Operation B1".to_string()
    }

    fn operation_b2(&self) -> String {
        "Subsystem B, Operation B2".to_string()
    }
}

struct Facade {
    subsystem_a: SubsystemA,
    subsystem_b: SubsystemB,
}

impl Facade {
    fn new() -> Self {
        Facade {
            subsystem_a: SubsystemA,
            subsystem_b: SubsystemB,
        }
    }

    fn operation(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.subsystem_a.operation_a1());
        result.push_str("\n");
        result.push_str(&self.subsystem_a.operation_a2());
        result.push_str("\n");
        result.push_str(&self.subsystem_b.operation_b1());
        result.push_str("\n");
        result.push_str(&self.subsystem_b.operation_b2());
        result
    }
}

fn main() {
    let facade = Facade::new();
    println!("{}", facade.operation());
}