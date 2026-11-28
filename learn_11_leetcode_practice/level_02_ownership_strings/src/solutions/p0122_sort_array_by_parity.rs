//! # Problem 0122: Sort Array By Parity
//!
//! - **Difficulty:** Easy
//! - **Topics:** owned `mut` parameter, write index, `slice::swap`
//! - **Source:** [LeetCode 905. Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)
//!
//! ## Problem
//!
//! Given a vector of non-negative integers `nums`, rearrange it so that **all even numbers come
//! before all odd numbers**, and return it. Any order that satisfies this rule is accepted (the
//! evens among themselves, and the odds among themselves, may appear in any order).
//!
//! Implement `Solution::sort_array_by_parity(nums: Vec<i32>) -> Vec<i32>`. The function owns
//! `nums`, so rearrange it in place (write the parameter as `mut nums`) and return it.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 1, 2, 4]
//! Output: [2, 4, 3, 1]      ([4, 2, 3, 1], [2, 4, 1, 3] and [4, 2, 1, 3] are also accepted)
//!
//! Input:  nums = [0]
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 5000`
//! - `0 <= nums[i] <= 5000`
//!
//! ## Hints
//!
//! 1. Keep an index `next_even`: the position where the next even number should go.
//! 2. Scan the vector; whenever you meet an even number, swap it into `next_even` and advance it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** everything before `next_even` is even. Scanning from left to right, each even number
//! found is swapped to position `next_even`, which then grows by one. Odd numbers are pushed to the
//! right by those swaps.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `mut nums: Vec<i32>` reuses the caller's allocation; returning it moves it back out.
//! - `slice::swap(a, b)` is safe even when `a == b`.
//! - An alternative is `nums.sort_by_key(|x| x % 2)`, which is `O(n log n)` but keeps the original
//!   order inside each group (it is a stable sort).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut next_even = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                nums.swap(next_even, i);
                next_even += 1;
            }
        }
        nums
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the rule (evens first) and that the result is a rearrangement of the input.
    fn assert_valid(input: Vec<i32>) {
        let mut expected = input.clone();
        let result = Solution::sort_array_by_parity(input);
        let first_odd = result.iter().position(|x| x % 2 == 1).unwrap_or(result.len());
        assert!(result[first_odd..].iter().all(|x| x % 2 == 1), "{result:?}");
        let mut sorted = result;
        sorted.sort();
        expected.sort();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn example_mixed() {
        assert_valid(vec![3, 1, 2, 4]);
    }

    #[test]
    fn example_single_zero() {
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }

    #[test]
    fn all_odd_or_all_even() {
        assert_eq!(Solution::sort_array_by_parity(vec![1, 3, 5]).len(), 3);
        assert_valid(vec![1, 3, 5]);
        assert_valid(vec![2, 4, 6]);
    }

    #[test]
    fn odd_numbers_first() {
        assert_valid(vec![5, 7, 9, 2, 0]);
    }

    #[test]
    fn large_input() {
        assert_valid((0..5000).rev().collect());
    }
}
