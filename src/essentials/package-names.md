Rust support underscore (_) for package name but not hypens (-). However you may see packages in Crates.io with heypes. 

[The related RFC](https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md) recommens explictly renaming those Crates with underscore. E.g.

```
extern crate "rustc-serialize" as rustc_serialize;
```

There is [quite bit of debate](https://www.reddit.com/r/rust/comments/194clzq/underscores_vs_dashes_in_crate_names/) whether hypens should not be used in library names.






