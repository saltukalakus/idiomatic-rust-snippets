### Bubble Sort Algorithm

Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted.

Here is a simple implementation of the bubble sort algorithm in Rust:

```rust
fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 0 {
        let mut new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

1. **Function Definition**: The `bubble_sort` function takes a mutable slice of integers.
2. **Outer Loop**: The outer `while` loop continues until the array is sorted. The variable `n` keeps track of the length of the unsorted portion of the array.
3. **Inner Loop**: The inner `for` loop iterates through the array, comparing adjacent elements.
4. **Swapping**: If the element at position `i-1` is greater than the element at position `i`, they are swapped using the `swap` method.
5. **Tracking Swaps**: The variable `new_n` keeps track of the last swap position, which helps in reducing the number of comparisons in subsequent passes.
6. **Main Function**: The `main` function demonstrates the usage of the `bubble_sort` function by sorting an example array and printing the results before and after sorting.

This implementation of bubble sort has a time complexity of O(n^2) in the worst and average cases, making it inefficient for large datasets. However, it is easy to understand and implement, making it a good choice for educational purposes.