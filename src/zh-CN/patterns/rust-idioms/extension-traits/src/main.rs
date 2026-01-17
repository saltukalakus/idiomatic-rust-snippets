// 扩展 trait for String/&str
trait StringExt {
    fn is_blank(&self) -> bool;
    fn truncate_with_ellipsis(&self, max_len: usize) -> String;
}

impl StringExt for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
    
    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &self[..max_len - 3])
        }
    }
}

// 扩展 trait for iterators
trait IteratorExt: Iterator {
    fn sum_by<B, F>(self, f: F) -> B
    where
        Self: Sized,
        B: std::iter::Sum,
        F: FnMut(Self::Item) -> B;
}

impl<I: Iterator> IteratorExt for I {
    fn sum_by<B, F>(self, mut f: F) -> B
    where
        B: std::iter::Sum,
        F: FnMut(Self::Item) -> B,
    {
        self.map(|item| f(item)).sum()
    }
}

// 扩展 trait for Option
trait OptionExt<T> {
    fn ok_or_else_log<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        E: std::fmt::Display;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_else_log<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        E: std::fmt::Display,
    {
        match self {
            Some(v) => Ok(v),
            None => {
                let e = err();
                eprintln!("Warning: {}", e);
                Err(e)
            }
        }
    }
}

// 扩展 trait for Vec
trait VecExt<T> {
    fn push_if_not_exists(&mut self, value: T)
    where
        T: PartialEq;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_if_not_exists(&mut self, value: T)
    where
        T: PartialEq,
    {
        if !self.contains(&value) {
            self.push(value);
        }
    }
}

fn main() {
    // 使用 StringExt
    println!("=== StringExt ===");
    let text = "   ";
    println!("'{}' is blank: {}", text, text.is_blank());
    
    let long_text = "这是一个需要截断的非常长的字符串";
    println!("Truncated: {}", long_text.truncate_with_ellipsis(20));
    
    // 使用 IteratorExt
    println!("\n=== IteratorExt ===");
    struct Item { price: i32 }
    let items = vec![
        Item { price: 10 },
        Item { price: 20 },
        Item { price: 30 },
    ];
    let total: i32 = items.iter().sum_by(|item| item.price);
    println!("总价格: {}", total);
    
    // 使用 OptionExt
    println!("\n=== OptionExt ===");
    let value: Option<i32> = None;
    let result = value.ok_or_else_log(|| "值 was not found");
    println!("结果: {:?}", result);
    
    // 使用 VecExt
    println!("\n=== VecExt ===");
    let mut numbers = vec![1, 2, 3];
    numbers.push_if_not_exists(2); // 不会添加 - 已存在
    numbers.push_if_not_exists(4); // 将添加
    println!("数字s: {:?}", numbers);
}
