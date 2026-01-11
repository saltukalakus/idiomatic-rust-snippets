// In Rust, the Visitor pattern is often better replaced with enums and pattern matching
// This provides exhaustive checking and avoids complex trait hierarchies

#[derive(Debug)]
enum Element {
    NumberNode(i32),
    TextNode(String),
    ListNode(Vec<Element>),
}

// Instead of a Visitor trait, we use regular functions or methods
// Pattern matching makes operations explicit and type-safe

impl Element {
    // Operation 1: Calculate a score
    fn calculate_score(&self) -> i32 {
        match self {
            Element::NumberNode(n) => *n * 2,
            Element::TextNode(s) => s.len() as i32 * 10,
            Element::ListNode(items) => items.iter().map(|e| e.calculate_score()).sum(),
        }
    }

    // Operation 2: Generate a description
    fn describe(&self) -> String {
        match self {
            Element::NumberNode(n) => format!("Number: {}", n),
            Element::TextNode(s) => format!("Text: '{}'", s),
            Element::ListNode(items) => {
                let descriptions: Vec<String> = items.iter().map(|e| e.describe()).collect();
                format!("List[{}]", descriptions.join(", "))
            }
        }
    }

    // Operation 3: Transform the element
    fn transform(&mut self) {
        match self {
            Element::NumberNode(n) => *n += 10,
            Element::TextNode(s) => *s = s.to_uppercase(),
            Element::ListNode(items) => {
                for item in items.iter_mut() {
                    item.transform();
                }
            }
        }
    }
}

fn main() {
    let mut tree = Element::ListNode(vec![
        Element::NumberNode(5),
        Element::TextNode("hello".to_string()),
        Element::ListNode(vec![
            Element::NumberNode(3),
            Element::TextNode("world".to_string()),
        ]),
    ]);

    println!("Initial description: {}", tree.describe());
    println!("Initial score: {}", tree.calculate_score());

    // Transform the tree
    tree.transform();

    println!("\nAfter transformation:");
    println!("Description: {}", tree.describe());
    println!("Score: {}", tree.calculate_score());
}

// If you absolutely need external operations (plugin-style), use trait objects:
// trait Visitor {
//     fn visit(&mut self, element: &Element);
// }
// But this is rarely needed - pattern matching is more idiomatic and performant