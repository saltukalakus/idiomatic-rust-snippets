````markdown
### 过度克隆（Excessive Cloning）

为了满足借用检查器而克隆数据是常见的初学者反模式。虽然 `.clone()` 可以快速解决所有权错误，但它会产生不必要的分配和复制。对于大型数据结构或紧密循环，这会显著影响性能。

理解 Rust 的借用规则可以在不克隆的情况下高效共享数据。引用是零成本抽象。

```rust, editable
fn analyze_text(text: String) -> (usize, usize) {
    let word_count = text.clone().split_whitespace().count();
    let char_count = text.clone().chars().count();
    let line_count = text.clone().lines().count();
    (word_count, line_count)
}

fn main() {
    let large_text = "word ".repeat(100_000);
    let (words, lines) = analyze_text(large_text);
    println!("Words: {}, Lines: {}", words, lines);
}
```

克隆字符串三次会为一次简单分析创建大约三个 600KB 的分配。每次 `.clone()` 都会将整个字符串复制到新的堆分配中。这个反模式将内存使用量人为放大 4 倍（原始 + 3 次克隆）。虽然现代系统可以处理，但在内存受限环境或需要处理大量文档时，这非常浪费。

下面的示例通过借用而非克隆来改进。

```rust, editable
fn analyze_text(text: &str) -> (usize, usize) {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    let line_count = text.lines().count();
    (word_count, line_count)
}

fn main() {
    let large_text = "word ".repeat(100_000);
    let (words, lines) = analyze_text(&large_text);
    println!("Words: {}, Lines: {}", words, lines);
}
```

**关键改进**：
- 接受 `&str` 而非 `String` —— 可与借用数据一起工作
- 分析过程中无额外分配 —— 仅读取原始数据
- 更低的内存占用 —— 无重复分配
- 更灵活的 API —— 可以接受 `&String`、`&str` 或字符串字面量
- 更好的缓存利用 —— 访问相同内存位置

````