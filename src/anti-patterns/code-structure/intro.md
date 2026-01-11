### Code Structure Anti-Patterns

Idiomatic Rust emphasizes expressive, clear code that leverages the language's features. Poor code structure can make Rust programs harder to read, maintain, and debug, even when they compile successfully.

This section covers common anti-patterns in code organization and style:

1- [Manual Indexing](./manual-indexing.md): Using manual index-based loops instead of iterator methods like `.iter()`, `.map()`, and `.filter()`.

2- [Poor Pattern Matching](./poor-pattern-matching.md): Using verbose `if-else` chains instead of elegant `match` expressions or `if let`.

3- [Not Using Iterators](./iterator-anti-patterns.md): Collecting iterators unnecessarily or using manual loops when iterator chains would be clearer and more efficient.

4- [Ignoring Compiler Warnings](./ignoring-warnings.md): Dismissing helpful compiler warnings that point to potential issues or non-idiomatic code.
