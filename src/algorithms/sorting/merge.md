### Merge Sort Algorithm

Merge sort is a divide-and-conquer algorithm that splits an array into halves, recursively sorts each half, and then merges the sorted halves to produce the final sorted array. For a visual explanation of the algorithm please visit [Programiz](https://www.programiz.com/dsa/merge-sort).


1. Split the array into two halves.
2. Recursively sort each half.
3. Merge the two sorted halves into a single sorted array.

```rust
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.clone_from_slice(&ret);
}

// Merge two subarrays left and right into ret array
fn merge<T: Ord + Clone>(left: &[T], right: &[T], ret: &mut [T]) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();
    let mut i = 0;

    // Until we reach either end of either left or right array, pick the smaller 
    // among the elements and place it in the correct position at the returned array
    while let (Some(left_val), Some(right_val)) = (left_peek, right_peek) {
        if left_val <= right_val {
            ret[i] = left_val.clone();
            left_peek = left_iter.next();
        } else {
            ret[i] = right_val.clone();
            right_peek = right_iter.next();
        }
        i += 1;
    }

    // At this point either left or right array may have elements 
    // not cloned to the returned array yet, those elements are 
    // cloned next.

    while let Some(left_val) = left_peek {
        ret[i] = left_val.clone();
        left_peek = left_iter.next();
        i += 1;
    }

    while let Some(right_val) = right_peek {
        ret[i] = right_val.clone();
        right_peek = right_iter.next();
        i += 1;
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

- `merge_sort` function: Recursively splits the array into halves and sorts each half.
- `merge` function: Merges two sorted slices into a single sorted slice.