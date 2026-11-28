# Problem 0829: Partition Equal Subset Sum

- **Difficulty:** Medium
- **Topics:** 0/1 knapsack (subset sum), iterating capacities backwards, `Vec<bool>`
- **Source:** [LeetCode 416. Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/)

## Problem

Given an array of positive integers `nums`, return `true` if it can be split into two groups
(every element in exactly one group) whose sums are equal, and `false` otherwise.

Implement `Solution::can_partition(nums: Vec<i32>) -> bool`.

## Examples

```text
Input:  nums = [1, 5, 11, 5]
Output: true
Explanation: [1, 5, 5] and [11].

Input:  nums = [1, 2, 3, 5]
Output: false
```

## Constraints

- `1 <= nums.len() <= 200`
- `1 <= nums[i] <= 100`

## Hints

1. An odd total can never be split evenly. Otherwise you need a subset that sums to `total / 2`.
2. `reach[s]` = "some subset of the elements seen so far sums to `s`". Each element is used at
   most once.
3. Update `reach` from the largest `s` down to the element's value, so an element is not
   counted twice in one pass.

## Starter code

Write your code in [`src/practice/p0829_partition_equal_subset_sum.rs`](../src/practice/p0829_partition_equal_subset_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0829
```

```rust
pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
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
    fn example_possible() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn example_impossible() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn single_and_pair() {
        assert!(!Solution::can_partition(vec![1]));
        assert!(Solution::can_partition(vec![2, 2]));
    }

    #[test]
    fn even_total_but_no_half() {
        // Total 8, but no subset sums to 4.
        assert!(!Solution::can_partition(vec![1, 2, 5]));
    }

    #[test]
    fn each_element_used_once() {
        // With reuse, 3 + 3 = 6 would "work"; without it there is no subset summing to 6.
        assert!(!Solution::can_partition(vec![3, 9]));
    }

    #[test]
    fn largest_input() {
        assert!(Solution::can_partition(vec![100; 200]));
        let mut nums = vec![100; 199];
        nums.push(1);
        assert!(!Solution::can_partition(nums));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** subset-sum DP. `reach[0] = true`. For each element `x`, for `s` from `half` down to
`x`: `reach[s] |= reach[s - x]`. The downward loop reads only values from *before* `x` was
considered, which is exactly the 0/1 (use at most once) rule. Stop early when `reach[half]`
becomes true.

**Complexity:** `O(n * total)` time, `O(total)` space.

**Rust concepts:**
- `(x..=half).rev()` is the backward capacity loop that distinguishes 0/1 knapsack from the
  unbounded version of problems 0809-0811.
- `iter().sum::<i32>()` needs the target type, given here by the turbofish.

```rust
pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let half = (total / 2) as usize;
        let mut reach = vec![false; half + 1];
        reach[0] = true;
        for x in nums.into_iter().map(|x| x as usize) {
            for s in (x..=half).rev() {
                reach[s] = reach[s] || reach[s - x];
            }
            if reach[half] {
                return true;
            }
        }
        reach[half]
    }
}
```

Full tested solution: [`src/solutions/p0829_partition_equal_subset_sum.rs`](../src/solutions/p0829_partition_equal_subset_sum.rs)

</details>
