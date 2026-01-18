Below is a **detailed, professional README** you can use for the project:

---

# 📊 Median and Mode Calculator in Rust

## Overview

This project demonstrates how to compute the **median** and **mode** of a list of integers in **Rust**, using two core data structures:

* **Vector (`Vec`)** → for storing and sorting numbers
* **HashMap (`HashMap`)** → for counting occurrences of values

It is a practical exercise that showcases Rust fundamentals such as ownership, borrowing, sorting, pattern matching, and safe data handling.

---

## Problem Statement

Given a list of integers:

1. **Median**

   * Sort the list
   * Return the value at the middle index
   * If the list length is odd, the median is the exact middle value
   * (Optional extension: average the two middle values if the length is even)

2. **Mode**

   * The value that occurs **most frequently**
   * If multiple values have the same highest frequency, one of them is returned

---

## Example

### Input

```text
[1, 2, 2, 3, 4]
```

### Output

```text
Median: 2
Mode: 2
```

---

## Project Goals

* Practice working with **collections** in Rust
* Understand **sorting and indexing** with vectors
* Learn how to use **HashMap** for frequency counting
* Apply Rust’s **ownership and borrowing rules**
* Write safe and idiomatic Rust code

---

## Technologies Used

* **Rust programming language**
* Standard Library:

  * `Vec`
  * `HashMap` from `std::collections`

---

## How It Works

### 1. Store the Numbers in a Vector

The input numbers are stored in a `Vec<i32>`.

```rust
let numbers = vec![1, 2, 2, 3, 4];
```

---

### 2. Calculate the Median

1. Clone the vector to avoid modifying the original data
2. Sort the cloned vector
3. Find the middle index
4. Return the value at that index

```rust
let mut sorted = nums.clone();
sorted.sort();

let mid = sorted.len() / 2;
let median = sorted[mid];
```

---

### 3. Calculate the Mode

1. Create a `HashMap` to store counts of each number
2. Iterate through the vector and update counts
3. Track the number with the highest frequency

```rust
let mut counts: HashMap<i32, usize> = HashMap::new();

for &num in nums {
    *counts.entry(num).or_insert(0) += 1;
}
```

---

## Complete Function Implementation

```rust
use std::collections::HashMap;

fn median_and_mode(nums: &Vec<i32>) -> (i32, i32) {
    // Calculate median
    let mut sorted = nums.clone();
    sorted.sort();
    let median = sorted[sorted.len() / 2];

    // Calculate mode
    let mut counts = HashMap::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut mode = nums[0];
    let mut max_count = 0;

    for (num, count) in counts {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }

    (median, mode)
}
```

---

## Example Usage

```rust
fn main() {
    let numbers = vec![1, 2, 2, 3, 4];

    let (median, mode) = median_and_mode(&numbers);

    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
```

---

## Assumptions and Limitations

* The input list is **non-empty**
* If multiple modes exist, **only one is returned**
* The median for even-length lists is the **upper middle value**

  * (This can be extended to return the average instead)

---

## Time and Space Complexity

| Operation            | Complexity   |
| -------------------- | ------------ |
| Sorting the vector   | `O(n log n)` |
| Counting frequencies | `O(n)`       |
| Overall              | `O(n log n)` |
| Extra space          | `O(n)`       |

---

## Possible Improvements

* Support floating-point numbers
* Handle empty input gracefully
* Return **all modes** if there is more than one
* Compute the average median for even-length lists
* Make the function generic over numeric types

---

## Learning Outcomes

By completing this project, you will understand:

* How to use vectors and hash maps together
* How Rust prevents unsafe memory access
* How to write clear, maintainable Rust code
* How to solve common data-processing problems in Rust

---

## License

This project is for **educational purposes** and is free to use, modify, and extend.

---
