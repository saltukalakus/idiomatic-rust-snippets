// The existing interface that we want to adapt
trait Target {
    fn request(&self) -> String;
}

// The adaptee class with a different interface
struct Adaptee;

impl Adaptee {
    fn specific_request(&self) -> String {
        "specific request".to_string()
    }
}

// The adapter class that makes Adaptee compatible with Target
struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        // Translate the interface of Adaptee to the Target interface
        self.adaptee.specific_request()
    }
}

fn main() {
    let adaptee = Adaptee;
    let adapter = Adapter::new(adaptee);

    // Client code can use the adapter as if it were a Target
    println!("Adapter request: {}", adapter.request());
}