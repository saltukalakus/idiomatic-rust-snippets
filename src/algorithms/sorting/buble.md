### Bubble Sort Algorithm

Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted. For a visual explanation of the algorithm please visit [Wikipedia](https://en.wikipedia.org/wiki/Bubble_sort).

```rust, editable
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
        println!("Iteration {:?} {}", arr, new_n);
        n = new_n;
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
    println!("Unsorted array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

- The `bubble_sort` function takes a mutable slice of integers.
- The outer `while` loop continues until the array is sorted. The variable `n` keeps track of the length of the unsorted portion of the array.
- The inner `for` loop iterates through the array, comparing adjacent elements.
- If the element at position `i-1` is greater than the element at position `i`, they are swapped using the `swap` method.
- The variable `new_n` keeps track of the last swap position, which helps in reducing the number of comparisons in subsequent passes.