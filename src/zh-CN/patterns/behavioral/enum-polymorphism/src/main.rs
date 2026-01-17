// 在 Rust 中，访问者模式通常最好用枚举和模式匹配来替代
// 这提供了详尽检查并避免了复杂的 trait 层次结构

#[derive(Debug)]
enum Element {
    NumberNode(i32),
    TextNode(String),
    ListNode(Vec<Element>),
}

// 我们使用常规函数或方法，而不是访问者 trait
// 模式匹配使操作显式且类型安全

impl Element {
    // 操作 1：计算分数
    fn calculate_score(&self) -> i32 {
        match self {
            Element::NumberNode(n) => *n * 2,
            Element::TextNode(s) => s.len() as i32 * 10,
            Element::ListNode(items) => items.iter().map(|e| e.calculate_score()).sum(),
        }
    }

    // 操作 2：生成描述
    fn describe(&self) -> String {
        match self {
            Element::NumberNode(n) => format!("数字: {}", n),
            Element::TextNode(s) => format!("文本: '{}'", s),
            Element::ListNode(items) => {
                let descriptions: Vec<String> = items.iter().map(|e| e.describe()).collect();
                format!("列表[{}]", descriptions.join(", "))
            }
        }
    }

    // 操作 3：转换元素
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
        Element::TextNode("你好".to_string()),
        Element::ListNode(vec![
            Element::NumberNode(3),
            Element::TextNode("世界".to_string()),
        ]),
    ]);

    println!("初始描述: {}", tree.describe());
    println!("初始分数: {}", tree.calculate_score());

    // 转换树结构
    tree.transform();

    println!("\n转换后:");
    println!("描述: {}", tree.describe());
    println!("分数: {}", tree.calculate_score());
}

// 如果确实需要外部操作（插件风格），使用 trait 对象:
// trait 访问者 {
//     fn visit(&mut self, element: &Element);
// }
// 但这很少需要——模式匹配更符合惯用法且性能更好
