use std::collections::HashMap;
use std::rc::Rc;

// The Flyweight struct that contains the shared state
struct Flyweight {
    shared_state: String,
}

impl Flyweight {
    fn new(shared_state: &str) -> Self {
        Flyweight {
            shared_state: shared_state.to_string(),
        }
    }

    fn operation(&self, unique_state: &str) {
        println!(
            "Flyweight: Displaying shared ({}) and unique ({}) state.",
            self.shared_state, unique_state
        );
    }
}

// The FlyweightFactory that creates and manages Flyweight objects
struct FlyweightFactory {
    flyweights: HashMap<String, Rc<Flyweight>>,
}

impl FlyweightFactory {
    fn new() -> Self {
        FlyweightFactory {
            flyweights: HashMap::new(),
        }
    }

    fn get_flyweight(&mut self, shared_state: &str) -> Rc<Flyweight> {
        if !self.flyweights.contains_key(shared_state) {
            println!("FlyweightFactory: Creating new flyweight for state '{}'.", shared_state);
            let flyweight = Rc::new(Flyweight::new(shared_state));
            self.flyweights.insert(shared_state.to_string(), flyweight);
        } else {
            println!("FlyweightFactory: Reusing existing flyweight for state '{}'.", shared_state);
        }
        Rc::clone(self.flyweights.get(shared_state).unwrap())
    }
}

fn main() {
    let mut factory = FlyweightFactory::new();

    let flyweight1 = factory.get_flyweight("shared1");
    flyweight1.operation("unique1");

    let flyweight2 = factory.get_flyweight("shared2");
    flyweight2.operation("unique2");

    let flyweight3 = factory.get_flyweight("shared1");
    flyweight3.operation("unique3");
}