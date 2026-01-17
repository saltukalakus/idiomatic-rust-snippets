### Option 枚举

`Option` 枚举表示一个值可能存在（`Some`）或不存在（`None`）。它通常用于处理可能缺失的值，避免使用空指针并降低空指针错误的风险。

### 定义

`Option` 枚举在标准库中定义为：

```rust,ignore
enum Option<T> {
    Some(T),
    None,
}
```

这里的 `T` 是泛型类型参数，表示 `Option` 可以包含任意类型的值。

### 基本用法

```rust, editable
fn find_word(word: &str) -> Option<usize> {
    let words = vec!["hello", "world", "rust"];
    words.iter().position(|&w| w == word)
}

fn main() {
    match find_word("rust") {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

`find_word` 返回 `Option<usize>`：若找到则返回 `Some(index)`，否则返回 `None`。

#### 使用 `unwrap`

```rust, editable
fn main() {
    let some_number = Some(10);
    let number = some_number.unwrap();
    println!("The number is: {}", number);
}
```

- `unwrap` 方法用于提取 `Some` 内的值，但在 `None` 上调用会导致 panic。仅在你确定 `Option` 为 `Some` 时使用。

#### 使用 `unwrap_or`

```rust, editable
fn main() {
    let some_number = Some(10);
    let none_number: Option<i32> = None;

    println!("The number is: {}", some_number.unwrap_or(0));
    println!("The number is: {}", none_number.unwrap_or(0));
}
```

- `unwrap_or` 在 `Option` 为 `None` 时提供默认值。
