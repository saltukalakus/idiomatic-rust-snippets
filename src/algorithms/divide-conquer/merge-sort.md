### Merge Sort Algorithm (WIP)

Merge sort is a divide-and-conquer algorithm that splits an array into halves, recursively sorts each half, and then merges the sorted halves to produce the final sorted array. 

Here's a simple implementation of merge sort in Rust:

```rust
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge(&mut left, &mut right, arr);
}

fn merge<T: Ord + Clone>(left: &mut Vec<T>, right: &mut Vec<T>, arr: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut arr_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            arr[arr_index] = left[left_index].clone();
            left_index += 1;
        } else {
            arr[arr_index] = right[right_index].clone();
            right_index += 1;
        }
        arr_index += 1;
    }

    while left_index < left.len() {
        arr[arr_index] = left[left_index].clone();
        left_index += 1;
        arr_index += 1;
    }

    while right_index < right.len() {
        arr[arr_index] = right[right_index].clone();
        right_index += 1;
        arr_index += 1;
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

1. **Base Case**: If the array has one or zero elements, it is already sorted.
2. **Divide**: Split the array into two halves.
3. **Conquer**: Recursively sort each half.
4. **Merge**: Merge the two sorted halves into a single sorted array.

The `merge` function combines two sorted vectors into a single sorted array by comparing the elements of both vectors and placing the smaller element into the array.
