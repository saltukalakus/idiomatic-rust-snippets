````markdown
### 包名和连字符（破折号）的一个怪癖

Rust 支持包名中使用下划线 (_)，但不支持连字符 (-)。但是，您可能会在 Crates.io 上看到带有连字符的包。

[相关的 RFC](https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md) 建议使用下划线明确重命名这些 crate。例如：


```
extern crate "rustc-serialize" as rustc_serialize;
```

关于库名中是否应使用连字符，存在[相当多的争论](https://www.reddit.com/r/rust/comments/194clzq/underscores_vs_dashes_in_crate_names/)。

避免使用连字符似乎是面向未来的最安全选择，但我同意使用破折号代替下划线的美学方面。
````
