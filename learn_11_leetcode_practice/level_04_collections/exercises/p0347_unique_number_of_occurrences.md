# Problem 0347: Unique Number of Occurrences

- **Difficulty:** Easy
- **Topics:** `HashMap` counts, `values()`, feeding values into a `HashSet`
- **Source:** [LeetCode 1207. Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/)

## Problem

Given a vector of integers `arr`, return `true` if every distinct value occurs a **different**
number of times (no two values share the same count), and `false` otherwise.

Implement `Solution::unique_occurrences(arr: Vec<i32>) -> bool`.

## Examples

```text
Input:  arr = [1, 2, 2, 1, 1, 3]
Output: true     (1 occurs 3 times, 2 twice, 3 once)

Input:  arr = [1, 2]
Output: false    (both occur once)

Input:  arr = [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]
Output: true
```

## Constraints

- `1 <= arr.len() <= 1000`
- `-1000 <= arr[i] <= 1000`

## Hints

1. First count the values with a `HashMap<i32, usize>`.
2. Then check that the counts themselves are all distinct: insert them into a `HashSet`.

## Starter code

Write your code in [`src/practice/p0347_unique_number_of_occurrences.rs`](../src/practice/p0347_unique_number_of_occurrences.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0347
```

```rust
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::unique_occurrences(vec![1, 2]));
    }

    #[test]
    fn example_three() {
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }

    #[test]
    fn single_value() {
        assert!(Solution::unique_occurrences(vec![5]));
        assert!(Solution::unique_occurrences(vec![5, 5, 5]));
    }

    #[test]
    fn large_staircase() {
        // Value k occurs k times for k = 1..=44 (990 elements): all counts differ.
        let mut arr: Vec<i32> = (1..=44)
            .flat_map(|k| std::iter::repeat_n(k, k as usize))
            .collect();
        assert!(Solution::unique_occurrences(arr.clone()));
        // One more copy of 1 makes value 1 occur twice, like value 2.
        arr.push(1);
        assert!(!Solution::unique_occurrences(arr));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two levels of hashing. The map turns values into counts; the set checks that no count
repeats. `insert` returning `false` signals a repeated count.

**Complexity:** `O(n)` average time and space.

**Rust concepts:**
- `counts.values()` iterates over `&usize` without the keys.
- `all(|&c| seen.insert(c))` stops at the first repeated count.
- Equivalent: `counts.len() == counts.values().collect::<HashSet<_>>().len()`.

```rust
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for x in arr {
            *counts.entry(x).or_insert(0) += 1;
        }
        let mut seen = HashSet::new();
        counts.values().all(|&c| seen.insert(c))
    }
}
```

Full tested solution: [`src/solutions/p0347_unique_number_of_occurrences.rs`](../src/solutions/p0347_unique_number_of_occurrences.rs)

</details>
