### Extension Traits

Extension traits allow you to add methods to types you don't own. This is a common pattern in Rust to extend functionality of external types (like those from `std` or other crates) without modifying them.

**Benefits**:
- Add methods to types from external crates
- Keep implementations organized and modular
- Work around Rust's orphan rule for traits
- Create domain-specific APIs on existing types

```rust, editable
{{#include extension-traits/src/main.rs}}
```

**Key Points**:
- The example defines `StringExt` trait with `is_blank()` and `truncate_with_ellipsis()` methods
- Implemented on `str` type (not owned by us) - adds methods like `.is_blank()` to all string slices
- `IteratorExt` adds `sum_by()` method to all iterators - maps items before summing
- `OptionExt` adds `ok_or_else_log()` that logs errors to stderr before returning `Err`
- `VecExt` adds `push_if_not_exists()` that only pushes if value not already in vector
- Users must `use` the trait to access extension methods

**Common Extension Traits in Rust Ecosystem**:
- `itertools::Itertools` - extends Iterator with many methods
- `futures::StreamExt` - async stream extensions
- `anyhow::Context` - extends Result with context methods

**When to Use**:
- Adding utility methods to standard library types
- Creating domain-specific APIs for existing types
- Providing convenience methods in your library
- When you can't modify the original type
