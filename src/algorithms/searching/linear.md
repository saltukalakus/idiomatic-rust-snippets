### Linear Search Algorithm

Linear search is a simple searching algorithm that checks each element in a list sequentially until the desired element is found or the list ends.

```rust, editable
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

- The `linear_search` function takes a slice of integers (`arr`) and a target integer (`target`) as arguments. 
- The function iterates over the array using `enumerate()` which provides both the index and the value of each element. For each element, it checks if the element is equal to the target. If a match is found, it returns the index wrapped in `Some`. If the loop completes without finding the target, the function returns `None`.