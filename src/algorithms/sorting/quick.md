### Quick Sort Algorithm

Quick sort is based on partitioning of an array of data into smaller arrays. A large array is partitioned into two arrays, one of which holds values smaller than the specified value, say pivot, based on which the partition is made and another array holds values greater than the pivot value.

1. Pick an element, called a pivot, from the array.
2. Partition the array into two sub-arrays:
    - Elements less than the pivot.
    - Elements greater than the pivot.
3. Recursively apply the above steps to the sub-arrays.

```rust
fn quick_sort(arr: &mut [i32]) {
     let len = arr.len();
     if len < 2 {
          return;
     }
     let pivot_index = partition(arr);
     println!("Left array: {:?}", &arr[0..pivot_index]);
     println!("Right array: {:?}", &arr[pivot_index + 1..len]);
     quick_sort(&mut arr[0..pivot_index]);
     quick_sort(&mut arr[pivot_index + 1..len]);
}

fn partition(arr: &mut [i32]) -> usize {
     let len = arr.len();

     // Pick the mid element as pivot
     let pivot_index = len / 2;
     arr.swap(pivot_index, len - 1);
     
     let mut store_index = 0;
     for i in 0..len - 1 {
          if arr[i] < arr[len - 1] {
                arr.swap(i, store_index);
                store_index += 1;
          }
     }
     arr.swap(store_index, len - 1);
     store_index
}

fn main() {
     let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
     println!("Original array: {:?}", arr);
     quick_sort(&mut arr);
     println!("Sorted array: {:?}", arr);
}
```

- `quick_sort` function: This function takes a mutable slice of integers and sorts it using the Quick Sort algorithm. It recursively sorts the sub-arrays.
- `partition` function: This function partitions the array around a pivot element and returns the index of the pivot after partitioning. Often it is better to select the mid item as the pivot if the array is already semi-sorted.