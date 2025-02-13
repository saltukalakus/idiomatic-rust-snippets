### Counting Sort Algorithm

Counting sort is an integer sorting algorithm that operates by counting the number of objects that have distinct key values. It is not a comparison-based sorting algorithm and is efficient when the range of input values is not significantly greater than the number of objects to be sorted.

1. **Find the range of the input values**: Determine the minimum and maximum values in the input array.
2. **Create a count array**: Create an array of size equal to the range of the input values, initialized to zero.
3. **Count the occurrences**: Iterate through the input array and count the occurrences of each value, storing the counts in the count array.
4. **Accumulate the counts**: Modify the count array such that each element at each index stores the sum of previous counts. This step helps in placing the elements in the correct position in the output array.
5. **Build the output array**: Iterate through the input array in reverse order, placing each element in its correct position in the output array based on the count array, and then decrease the count by one.

```rust
fn counting_sort(arr: &mut [usize], max_value: usize) {
    let mut count = vec![0; max_value + 1];
    let mut output = vec![0; arr.len()];

    // Count the occurrences of each value
    for &num in arr.iter() {
        count[num] += 1;
    }
    println!("Count array: {:?}", count);

    // Accumulate the counts
    for i in 1..=max_value {
        count[i] += count[i - 1];
    }
    println!("Accumulated count array: {:?}", count);

    // Build the output array
    for &num in arr.iter().rev() {
        println!("Current number being processed: {}", num);
        output[count[num] - 1] = num;
        println!("Output: {:?}", output);
        count[num] -= 1;
    }

    // Copy the sorted elements back to the original array
    arr.copy_from_slice(&output);
}

fn main() {
    let mut arr = [4, 2, 2, 8, 3, 3, 1];
    let max_value = *arr.iter().max().unwrap();
    counting_sort(&mut arr, max_value);
    println!("Sorted array: {:?}", arr);
}
```

- The `counting_sort` function takes a mutable slice of integers `arr` and the maximum value `max_value` in the array.
- It initializes a `count` array to store the count of each unique value and an `output` array to store the sorted elements.
- It iterates through the input array to count the occurrences of each value.
- It modifies the `count` array to accumulate the counts.
- It iterates through the input array in reverse order to place each element in its correct position in the `output` array.
- Finally, it copies the sorted elements from the `output` array back to the original array.

This implementation assumes that the input array contains non-negative integers and that the maximum value is known or can be determined.
