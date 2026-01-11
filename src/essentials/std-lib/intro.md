### Standard Library

This section explains some of the common standard library APIs and their usage for everyday programming tasks. The intention is not to cover the entire API, but to provide practical examples of frequently used functionality.

1- [Result Type](./result.md): Rust's primary mechanism for handling operations that can fail with explicit error information.

2- [Option Enum](./option.md): Enum representing a value that can be something (`Some`) or nothing (`None`), avoiding null pointer issues.

3- [String vs &str](./string-vs-str.md): Understanding the critical differences between owned String and borrowed &str types.

4- [Vec](./vec.md): Growable array type for dynamic collections that can grow or shrink at runtime.

5- [HashMap](./hashmap.md): Collection that stores key-value pairs for efficient lookups by key.

6- [Box](./box.md): Heap allocation smart pointer for large data, trait objects, and recursive types.

7- [Rc](./rc.md): Reference counting pointer for shared ownership of data in single-threaded contexts.

8- [Arc and Mutex](./arc-mutex.md): Thread-safe reference counting and mutual exclusion for concurrent programming.

9- [RefCell](./refcell.md): Interior mutability pattern that allows mutable borrows checked at runtime.

10- [Self as Type](./self-as-type.md): Using `Self` as a type alias in trait implementations.

11- [self vs Self](./self-vs-self.md): Understanding the difference between lowercase `self` and uppercase `Self`.

12- [Self](./self.md): The `self` keyword in method definitions and implementations.

13- [Some](./some.md): The `Some` variant of the `Option` enum for wrapping values.

14- [New](./new.md): Convention for constructor functions in Rust.

15- [Derive(Debug)](./drive(debug).md): Automatically implementing the Debug trait for custom types.

16- [Impl](./imp.md): Implementing methods and associated functions for types. 