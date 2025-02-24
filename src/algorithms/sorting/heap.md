### Heap Sort Algorithm

Heap sort is a comparison-based sorting algorithm that uses a [binary heap data structure](https://www.programiz.com/dsa/heap-data-structure). It divides its input into a sorted and an unsorted region and iteratively shrinks the unsorted region by extracting the largest element and moving that to the sorted region. For a visual explanation of the algorithm please visit [Programiz](https://www.programiz.com/dsa/heap-sort).

1. Build a max heap from the input data.
2. At this point, the largest item is stored at the root of the heap. Replace it with the last item of the heap followed by reducing the size of the heap by one. Finally, heapify the root of the tree.
3. Repeat step 2 while the size of the heap is greater than 1.

```rust
fn heapify(arr: &mut [i32], n: usize, i: usize) {
    // Find largest among root, left child and right child
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // Swap and continue heapifying if root is not largest
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // At this point arr[0] holds the larger item
    println!("Max heap array: {:?}", arr);

    // Heap sort, 
    for i in (1..n).rev() {
        arr.swap(0, i);
        // Heapify root element to get highest element at root again
        heapify(arr, i, 0);
    } 
} 

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
    println!("Original array: {:?}", arr);
    heap_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
```

- The `heapify` function ensures that the subtree rooted at index `i` is a max heap. It compares the root with its children and swaps them if necessary, then recursively heapifies the affected subtree.
- The `heap_sort` function first builds a max heap from the input array. It then repeatedly extracts the maximum element from the heap and places it at the end of the array, reducing the heap size each time.