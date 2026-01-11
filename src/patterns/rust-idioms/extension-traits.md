### Extension Traits

Extension traits allow you to add methods to types you don't own. This is a common pattern in Rust to extend functionality of external types (like those from `std` or other crates) without modifying them.

**Benefits**:
- Add methods to types from external crates
- Keep implementations organized and modular
- Work around Rust's orphan rule for traits
- Create domain-specific APIs on existing types

```rust
{{#include extension-traits/src/main.rs}}
```

**Key Points**:
- Define a trait with the methods you want to add
- Implement the trait for the external type
- Users must import the trait to use the extension methods
- Convention: name the trait `{TypeName}Ext`
- Use `Self: Sized` bound for methods that consume self

**Common Extension Traits in Rust Ecosystem**:
- `itertools::Itertools` - extends Iterator with many methods
- `futures::StreamExt` - async stream extensions
- `anyhow::Context` - extends Result with context methods

**When to Use**:
- Adding utility methods to standard library types
- Creating domain-specific APIs for existing types
- Providing convenience methods in your library
- When you can't modify the original type
