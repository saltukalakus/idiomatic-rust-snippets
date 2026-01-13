### Binary Search Algorithm

Binary search is an efficient algorithm for finding an item from a sorted list of items. It works by repeatedly dividing in half the portion of the list that could contain the item, until you've narrowed down the possible locations to just one.

This implementation assumes that the input array is sorted. If the array is not sorted, the binary search algorithm will not work correctly. Check out the [sorting section](../sorting/intro.html) for implementations of some of array sorting algorithms.

```rust, editable
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 6;
    match binary_search(&arr, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
```

- The `binary_search` function takes a slice of integers and a target integer to search for.
- It initializes two pointers, `low` and `high`, to the start and end of the array, respectively.
- It enters a loop where it calculates the middle index and compares the middle element with the target.
- If the middle element is equal to the target, it returns the index.
- If the middle element is less than the target, it adjusts the `low` pointer to search the right half of the array.
- If the middle element is greater than the target, it adjusts the `high` pointer to search the left half of the array.
- If the target is not found, it returns `None`.


