### Linear Search Algorithm (WIP)

Linear search is a simple searching algorithm that checks each element in a list sequentially until the desired element is found or the list ends.

Here is a simple implementation of the linear search algorithm in Rust:

```rust
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 7, 9, 11];
    let target = 7;

    match linear_search(&numbers, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
```

1. **Function Definition**: The `linear_search` function takes a slice of integers (`arr`) and a target integer (`target`) as arguments. It returns an `Option<usize>` which is `Some(index)` if the target is found, or `None` if it is not.

2. **Iteration**: The function iterates over the array using `enumerate()` which provides both the index and the value of each element.

3. **Comparison**: For each element, it checks if the element is equal to the target. If a match is found, it returns the index wrapped in `Some`.

4. **Return None**: If the loop completes without finding the target, the function returns `None`.

5. **Main Function**: In the `main` function, we define an array of numbers and a target value. We call the `linear_search` function and handle the result using a `match` statement to print whether the target was found and its index, or that it was not found.

This example demonstrates a basic linear search in Rust, which is straightforward and easy to understand.