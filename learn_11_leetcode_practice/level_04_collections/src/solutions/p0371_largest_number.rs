//! # Problem 0371: Largest Number
//!
//! - **Difficulty:** Medium
//! - **Topics:** a comparator that compares concatenations, `Iterator::cmp` on chained byte iterators, `concat`
//! - **Source:** [LeetCode 179. Largest Number](https://leetcode.com/problems/largest-number/)
//!
//! ## Problem
//!
//! Given a vector of non-negative integers `nums`, arrange them (as decimal strings, glued
//! together) to form the **largest** possible number, and return that number as a `String`. The
//! result must not have leading zeros, except for the number `"0"` itself.
//!
//! Implement `Solution::largest_number(nums: Vec<i32>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [10, 2]
//! Output: "210"
//!
//! Input:  nums = [3, 30, 34, 5, 9]
//! Output: "9534330"
//!
//! Input:  nums = [0, 0]
//! Output: "0"          (not "00")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `0 <= nums[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. Numeric order fails (`[10, 2]` must give `"210"`), and no simple per-number key works
//!    either. Compare **pairs** of numbers instead.
//! 2. `a` should come before `b` exactly when the string `a + b` is larger than `b + a`.
//! 3. If the largest piece is `"0"`, every piece is `"0"`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sort the decimal strings with the comparator "`a` before `b` if `ab > ba`". This
//! relation is a valid total order (it is transitive), and concatenating in that order gives the
//! largest number. Finally collapse an all-zero answer to `"0"`.
//!
//! **Complexity:** `O(n log n * d)` time where `d <= 10` is the number of digits, `O(n * d)` space.
//!
//! **Rust concepts:**
//! - `b.bytes().chain(a.bytes()).cmp(a.bytes().chain(b.bytes()))` compares the two concatenations
//!   lexicographically **without allocating** them (`Iterator::cmp`). Writing `b + a` vs `a + b`
//!   swaps the order, which sorts in descending order of "goodness".
//! - Both concatenations have the same length, so lexicographic order equals numeric order.
//! - `parts.concat()` glues a `Vec<String>` into one `String`.
//! - `parts.first().is_some_and(|p| p == "0")` tests the first piece without allocating a
//!   `String` to compare against.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        // @solution-start
        let mut parts: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        parts.sort_unstable_by(|a, b| b.bytes().chain(a.bytes()).cmp(a.bytes().chain(b.bytes())));
        if parts.first().is_some_and(|p| p == "0") {
            return "0".to_string();
        }
        parts.concat()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }

    #[test]
    fn example_three_all_zeros() {
        assert_eq!(Solution::largest_number(vec![0, 0]), "0");
        assert_eq!(Solution::largest_number(vec![0]), "0");
    }

    #[test]
    fn tricky_prefixes() {
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121");
        assert_eq!(Solution::largest_number(vec![824, 8247]), "8248247");
        assert_eq!(Solution::largest_number(vec![0, 1]), "10");
    }

    #[test]
    fn large_values() {
        assert_eq!(
            Solution::largest_number(vec![1_000_000_000, 999_999_999, 9]),
            "99999999991000000000"
        );
        let nums = vec![1; 100];
        assert_eq!(Solution::largest_number(nums), "1".repeat(100));
    }
}
