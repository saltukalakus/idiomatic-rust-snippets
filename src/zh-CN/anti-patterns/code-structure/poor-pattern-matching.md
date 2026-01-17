### 糟糕的模式匹配

过度复杂的 `match` 分支、重复的分支或忽视穷尽性会降低代码可读性并引入错误。应优先使用明确、简洁且可维护的匹配逻辑。

反模式示例：重复分支或冗长匹配：

```rust, editable
match option {
    Some(1) => println!("一"),
    Some(2) => println!("二"),
    Some(3) => println!("三"),
    _ => println!("other"),
}
```

如果只是处理范围或对多个模式做相同处理，可以合并分支或使用条件守卫：

```rust, editable
match option {
    Some(1..=3) => println!("一 to 三"),
    Some(_) => println!("other"),
    None => println!("none"),
}
```

避免在 `match` 中做大量逻辑；倾向于将复杂处理提取成函数以保持分支清晰。