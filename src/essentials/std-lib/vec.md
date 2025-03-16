### Vec type and vec! macro

`Vec` is a growable array type. The items in vectors must have the same type. It comes with a macro `vec!` to create vectors. 

Vec is particularly useful when you need a dynamic collection that can grow or shrink at runtime.

```rust
// Creating vectors
let v1: Vec<i32> = Vec::new();     // empty vector
let v2 = vec![1, 2, 3];            // vector with initial values
let v3 = vec!["aa", "dd", "cc"];   // vector with string type items

// Adding elements
let mut v4 = Vec::new();
v4.push(4);                        // [4]
v4.push(5);                        // [4, 5]


// Accessing elements
let third = v2[2];                 // 3
let first = v2.get(0);             // Some(1)

// Vector length
println!("Length: {}", v2.len());

// Vector contents
println!("Vector 1: {:?}", v1);
println!("Vector 2: {:?}", v2);
println!("Vector 3: {:?}", v3);
println!("Vector 3: {:?}", v4);
```

