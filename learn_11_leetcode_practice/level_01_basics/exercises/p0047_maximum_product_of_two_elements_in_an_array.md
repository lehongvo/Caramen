# Problem 0047: Maximum Product of Two Elements in an Array

- **Difficulty:** Easy
- **Topics:** tracking the two largest values in one pass, `if` / `else if`
- **Source:** [LeetCode 1464. Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)

## Problem

Given a vector of positive integers `nums`, choose two **different** indices `i` and `j` and
compute `(nums[i] - 1) * (nums[j] - 1)`. Return the largest value this expression can take.

Implement `Solution::max_product(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [3, 4, 5, 2]
Output: 12
Explanation: pick 4 and 5: (4 - 1) * (5 - 1) = 12.

Input:  nums = [1, 5, 4, 5]
Output: 16
Explanation: the two 5s are at different indices: (5 - 1) * (5 - 1) = 16.

Input:  nums = [3, 7]
Output: 12
```

## Constraints

- `2 <= nums.len() <= 500`
- `1 <= nums[i] <= 10^3`

## Hints

1. All values are at least 1, so every factor `nums[i] - 1` is non-negative. Bigger values give bigger products.
2. You only need the largest and the second largest value. Sorting works, but one pass is enough.

## Starter code

Write your code in [`src/practice/p0047_maximum_product_of_two_elements_in_an_array.rs`](../src/practice/p0047_maximum_product_of_two_elements_in_an_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0047
```

```rust
pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
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
    fn example_distinct() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn example_duplicate_maximum() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn example_two_elements() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
        assert_eq!(Solution::max_product(vec![1, 1]), 0);
    }

    #[test]
    fn maximum_first_or_last() {
        assert_eq!(Solution::max_product(vec![10, 2, 3, 9]), 72);
        assert_eq!(Solution::max_product(vec![2, 3, 9, 10]), 72);
    }

    #[test]
    fn largest_values() {
        let mut nums = vec![1; 500];
        nums[100] = 1000;
        nums[400] = 999;
        assert_eq!(Solution::max_product(nums), 999 * 998);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep `first >= second`, the two largest values seen so far. A new value larger than
`first` pushes `first` down to `second`; a value between them replaces `second`. Duplicates are
handled because the comparison with `second` uses `>`, and a second copy of the maximum is not
larger than `first` but is larger than the old `second`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- Updating two related variables in the right order: `second = first;` must happen before
  `first = x;`.
- The alternative `let mut v = nums; v.sort_unstable();` and reading the last two elements is
  `O(n log n)` but also fine for 500 elements.

```rust
pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        for &x in &nums {
            if x > first {
                second = first;
                first = x;
            } else if x > second {
                second = x;
            }
        }
        (first - 1) * (second - 1)
    }
}
```

Full tested solution: [`src/solutions/p0047_maximum_product_of_two_elements_in_an_array.rs`](../src/solutions/p0047_maximum_product_of_two_elements_in_an_array.rs)

</details>
