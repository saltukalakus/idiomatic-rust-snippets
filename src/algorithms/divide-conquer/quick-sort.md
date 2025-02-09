### Quick Sort Algorithm (WIP)

Quick sort is a highly efficient sorting algorithm and is based on partitioning an array into smaller sub-arrays. A large array is partitioned into two arrays, one of which holds values smaller than the specified value, say pivot, based on which the partition is made, and another array holds values greater than the pivot value.

### How Quick Sort Works

1. **Pick an element as pivot**: This can be any element from the array, commonly the first, last, or a random element.
2. **Partitioning**: Rearrange the array so that elements less than the pivot are on the left, elements greater than the pivot are on the right.
3. **Recursively apply**: Apply the above steps to the sub-arrays of elements with smaller and larger values.

```rust
fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len / 2];
    let mut left = 0;
    let mut right = len - 1;

    loop {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right = right.wrapping_sub(1);
        }
        if left >= right {
            return right;
        }
        arr.swap(left, right);
        left += 1;
        right = right.wrapping_sub(1);
    }
}

fn main() {
    let mut arr = [34, 7, 23, 32, 5, 62];
    println!("Original array: {:?}", arr);
    quick_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

- **quick_sort**: This function takes a mutable slice of integers and sorts it using the quick sort algorithm. It recursively sorts the sub-arrays.
- **partition**: This function partitions the array around a pivot element and returns the index of the pivot after partitioning.
- **main**: This function demonstrates the usage of the quick sort algorithm by sorting an example array.

By running the above code, you will see the original and sorted arrays printed to the console.
