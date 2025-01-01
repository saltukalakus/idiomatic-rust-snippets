#[derive(Clone)]
struct Prototype {
    field1: String,
    field2: i32,
}

impl Prototype {
    fn new(field1: String, field2: i32) -> Self {
        Prototype { field1, field2 }
    }

    fn clone_prototype(&self) -> Self {
        self.clone()
    }
}

fn main() {
    let original = Prototype::new(String::from("Prototype"), 42);
    let cloned = original.clone_prototype();

    println!("Original: {} - {}", original.field1, original.field2);
    println!("Cloned: {} - {}", cloned.field1, cloned.field2);
}