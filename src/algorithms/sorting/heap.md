### Heap Sort Algorithm (WIP)

Heap sort is a comparison-based sorting algorithm that uses a binary heap data structure. It divides its input into a sorted and an unsorted region and iteratively shrinks the unsorted region by extracting the largest element and moving that to the sorted region.

### Steps of Heap Sort

1. Build a max heap from the input data.
2. At this point, the largest item is stored at the root of the heap. Replace it with the last item of the heap followed by reducing the size of the heap by one. Finally, heapify the root of the tree.
3. Repeat step 2 while the size of the heap is greater than 1.

Below is a simple implementation of the heap sort algorithm in Rust:

```rust
fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn main() {
    let mut arr = [4, 10, 3, 5, 1];
    println!("Original array: {:?}", arr);
    heap_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

1. **heapify**: This function ensures that the subtree rooted at index `i` is a max heap. It compares the root with its children and swaps them if necessary, then recursively heapifies the affected subtree.
2. **heap_sort**: This function first builds a max heap from the input array. It then repeatedly extracts the maximum element from the heap and places it at the end of the array, reducing the heap size each time.
3. **main**: This function demonstrates the usage of the `heap_sort` function with a sample array.

By following these steps, you can sort an array using the heap sort algorithm in Rust.