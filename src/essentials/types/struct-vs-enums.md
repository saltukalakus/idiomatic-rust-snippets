### What is the differences between Structs vs Enums?

Enums and structs are both ways to define custom data types in Rust, but they serve different purposes and have different characteristics. Here’s a detailed comparison of enums and structs:

[**Structs**](./struct.md)

Structs are used to group related data together. They are similar to classes in object-oriented languages but without methods for encapsulation. Structs are typically used to represent entities with a fixed set of attributes.

### Characteristics of Structs

**Fixed Set of Fields**: Structs have a fixed set of fields, each with a name and a type.<br/>
**Homogeneous Data**: All instances of a struct have the same fields.<br/>
**Data Grouping**: Structs are used to group related data together.<br/>

[**Enums**](./enum.md)

Enums are used to define a type that can be one of several different variants. Each variant can have associated data. Enums are typically used to represent a value that could be one of several different types.

### Characteristics of Enums

**Multiple Variants**: Enums can have multiple variants, each representing a different possible value.<br/>
**Heterogeneous Data**: Each variant of an enum can have different types and amounts of associated data.<br/>
**Pattern Matching**: Enums are often used with pattern matching to handle different cases.<br/>
