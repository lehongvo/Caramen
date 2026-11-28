# Problem 0719: Subarray Product Less Than K

- **Difficulty:** Medium
- **Topics:** sliding window that counts subarrays, `i64` products, guard clauses
- **Source:** [LeetCode 713. Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k/)

## Problem

Given a vector `nums` of positive integers and an integer `k`, return how many **contiguous**
subarrays have a product of all their elements **strictly less** than `k`.

Implement `Solution::num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [10, 5, 2, 6], k = 100
Output: 8             ([10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6])

Input:  nums = [1, 2, 3], k = 0
Output: 0
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `1 <= nums[i] <= 1000`
- `0 <= k <= 10^6`

## Hints

1. All values are at least 1, so extending a subarray never decreases its product.
2. For each right end, find the smallest left end that keeps the product below `k`. How many
   valid subarrays end at `right` then?

## Starter code

Write your code in [`src/practice/p0719_subarray_product_less_than_k.rs`](../src/practice/p0719_subarray_product_less_than_k.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0719
```

```rust
pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
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
    fn example_eight_subarrays() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }

    #[test]
    fn example_k_zero() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }

    #[test]
    fn k_one_allows_nothing() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 1),
            0
        );
    }

    #[test]
    fn element_equal_to_k() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![5, 1, 5], 5),
            1
        );
    }

    #[test]
    fn all_ones_count_every_subarray() {
        // 30_000 * 30_001 / 2 subarrays.
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1; 30_000], 2),
            450_015_000
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sliding window that counts: for every `right`, keep the longest window
`left..=right` whose product is below `k` (shrink from the left while the product is too big).
Every subarray that ends at `right` and starts at `left` or later is also valid (it is a
sub-window), and there are exactly `right + 1 - left` of them. Summing that over all `right`
counts every valid subarray exactly once, by its right end.

If `k <= 1`, no product of positive integers is below `k`; returning early also prevents the
shrinking loop from running past `right`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- The product stays below `10^6` before each multiplication by at most `1000`, so it fits in
  `i32`, but `i64` makes that reasoning unnecessary.
- A guard clause (`if k <= 1 { return 0; }`) keeps the main loop free of special cases.

```rust
pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let k = i64::from(k);
        let (mut left, mut product, mut count) = (0, 1i64, 0);
        for (right, &x) in nums.iter().enumerate() {
            product *= i64::from(x);
            while product >= k {
                product /= i64::from(nums[left]);
                left += 1;
            }
            count += right + 1 - left; // subarrays ending at `right`
        }
        count as i32
    }
}
```

Full tested solution: [`src/solutions/p0719_subarray_product_less_than_k.rs`](../src/solutions/p0719_subarray_product_less_than_k.rs)

</details>
