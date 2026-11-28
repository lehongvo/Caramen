# Problem 0749: Sum of Subarray Minimums

- **Difficulty:** Medium
- **Topics:** previous / next smaller element with monotonic stacks, contribution counting, modular sums
- **Source:** [LeetCode 907. Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums/)

## Problem

Given a vector `arr`, consider every non-empty contiguous subarray and take its minimum.
Return the sum of all these minimums. The sum can be huge, so return it modulo
`1_000_000_007`.

Implement `Solution::sum_subarray_mins(arr: Vec<i32>) -> i32`.

## Examples

```text
Input:  arr = [3, 1, 2, 4]
Output: 17
        (subarrays: [3] [1] [2] [4] [3,1] [1,2] [2,4] [3,1,2] [1,2,4] [3,1,2,4]
         minimums:   3   1   2   4    1     1     2      1       1        1   -> sum 17)

Input:  arr = [11, 81, 94, 43, 3]
Output: 444
```

## Constraints

- `1 <= arr.len() <= 3 * 10^4`
- `1 <= arr[i] <= 3 * 10^4`

## Hints

1. Turn the question around: for each element, in how many subarrays is it the minimum?
2. That count is (choices for the left end) times (choices for the right end), limited by the
   nearest smaller elements on each side. Monotonic stacks find them in `O(n)`.
3. With equal values, make sure each subarray is credited to exactly one of them: use "strictly
   smaller" on one side and "smaller or equal" on the other.

## Starter code

Write your code in [`src/practice/p0749_sum_of_subarray_minimums.rs`](../src/practice/p0749_sum_of_subarray_minimums.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0749
```

```rust
pub struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
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
    fn example_four_values() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn example_five_values() {
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }

    #[test]
    fn single_value() {
        assert_eq!(Solution::sum_subarray_mins(vec![1]), 1);
    }

    #[test]
    fn equal_values_are_counted_once() {
        // Six subarrays, each with minimum 2.
        assert_eq!(Solution::sum_subarray_mins(vec![2, 2, 2]), 12);
    }

    #[test]
    fn increasing_values() {
        // 1 * 4 + 2 * 3 + 3 * 2 + 4 * 1
        assert_eq!(Solution::sum_subarray_mins(vec![1, 2, 3, 4]), 20);
    }

    #[test]
    fn large_input_needs_the_modulo() {
        // 30_000 * (30_000 * 30_001 / 2) = 13_500_450_000_000, modulo 1_000_000_007.
        assert_eq!(
            Solution::sum_subarray_mins(vec![30_000; 30_000]),
            449_905_500
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** contribution counting with monotonic stacks: element `i` is the minimum of the
subarrays that start after the previous element **strictly smaller** than `arr[i]` and end
before the next element **smaller or equal** to it. With `left[i]` choices for the start and
`right[i]` for the end, `arr[i]` contributes `arr[i] * left[i] * right[i]`.

- Left pass: keep a stack of indices with increasing values; pop while the top is `>= arr[i]`.
  The remaining top is the previous strictly smaller element (or none: `left = i + 1`).
- Right pass (from the end): pop while the top is `> arr[i]`; the remaining top is the next
  smaller-or-equal element (or none: `right = n - i`).

The asymmetric tie rule credits a subarray with several equal minima to the rightmost of them
only, so nothing is counted twice.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- Products are computed in `u64` and reduced modulo `1_000_000_007` (`MOD`, a `const`) after
  each addition; `3 * 10^4 * (3 * 10^4)^2` would overflow `i32` immediately.
- `iter().enumerate().rev()` walks indices from right to left.

```rust
pub struct Solution;

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = vec![0usize; n]; // number of possible start positions
        let mut right = vec![0usize; n]; // number of possible end positions
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        for (i, &x) in arr.iter().enumerate() {
            while stack.pop_if(|j| arr[*j] >= x).is_some() {}
            left[i] = stack.last().map_or(i + 1, |&j| i - j);
            stack.push(i);
        }
        stack.clear();
        for (i, &x) in arr.iter().enumerate().rev() {
            while stack.pop_if(|j| arr[*j] > x).is_some() {}
            right[i] = stack.last().map_or(n - i, |&j| j - i);
            stack.push(i);
        }
        // x * left * right <= 3 * 10^4 * (3 * 10^4)^2, which fits easily in u64.
        let total = arr.iter().enumerate().fold(0u64, |acc, (i, &x)| {
            (acc + x as u64 * (left[i] * right[i]) as u64) % MOD
        });
        total as i32
    }
}
```

Full tested solution: [`src/solutions/p0749_sum_of_subarray_minimums.rs`](../src/solutions/p0749_sum_of_subarray_minimums.rs)

</details>
