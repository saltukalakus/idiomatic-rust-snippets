trait Component {
    fn operation(&self) -> String;
}

struct Leaf {
    name: String,
}

impl Leaf {
    fn new(name: &str) -> Self {
        Leaf {
            name: name.to_string(),
        }
    }
}

impl Component for Leaf {
    fn operation(&self) -> String {
        format!("Leaf {}", self.name)
    }
}

struct Composite {
    name: String,
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn new(name: &str) -> Self {
        Composite {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }
}

impl Component for Composite {
    fn operation(&self) -> String {
        let mut result = format!("Composite {} contains:\n", self.name);
        for child in &self.children {
            result.push_str(&format!("  {}\n", child.operation()));
        }
        result
    }
}

fn main() {
    let mut root = Composite::new("root");

    let leaf1 = Box::new(Leaf::new("Leaf 1"));
    let leaf2 = Box::new(Leaf::new("Leaf 2"));

    let mut sub_tree = Composite::new("sub-tree");
    let leaf3 = Box::new(Leaf::new("Leaf 3"));
    sub_tree.add(leaf3);

    root.add(leaf1);
    root.add(leaf2);
    root.add(Box::new(sub_tree));

    println!("{}", root.operation());
}