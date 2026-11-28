//! # Problem 0190: Sort Colors
//!
//! - **Difficulty:** Medium
//! - **Topics:** three indices on one `&mut Vec<i32>` (Dutch national flag), `swap`, `match` on values
//! - **Source:** [LeetCode 75. Sort Colors](https://leetcode.com/problems/sort-colors/)
//!
//! ## Problem
//!
//! `nums` contains only the values `0`, `1` and `2` (red, white and blue). Sort it **in place** so
//! that all `0`s come first, then all `1`s, then all `2`s. Do not use the library's sort function;
//! aim for a single pass with `O(1)` extra space.
//!
//! Implement `Solution::sort_colors(nums: &mut Vec<i32>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 0, 2, 1, 1, 0]
//! Output: nums = [0, 0, 1, 1, 2, 2]
//!
//! Input:  nums = [2, 0, 1]
//! Output: nums = [0, 1, 2]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 300`
//! - `nums[i]` is `0`, `1` or `2`.
//!
//! ## Hints
//!
//! 1. Keep three regions: `[0, low)` holds 0s, `[low, mid)` holds 1s, `[high, n)` holds 2s, and
//!    `[mid, high)` is still unknown.
//! 2. Look at `nums[mid]`: a `0` is swapped to `low`, a `1` stays, a `2` is swapped to `high - 1`.
//! 3. After swapping with `high - 1`, do **not** advance `mid`: the value that arrived is unknown.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** Dijkstra's Dutch national flag partition. Each step shrinks the unknown region
//! `[mid, high)` by one: zeros are swapped into the front region, ones are left in the middle, twos
//! are swapped into the back region. When the unknown region is empty, the vector is sorted.
//!
//! **Complexity:** `O(n)` time (one pass), `O(1)` extra space.
//!
//! **Rust concepts:**
//! - Several `usize` indices into one vector are just numbers; only `swap` borrows the vector, once
//!   per call.
//! - Using an exclusive `high` (starting at `n`) avoids underflow when the vector is all twos.
//! - `match nums[mid] { 0 => ..., 1 => ..., _ => ... }` reads the value (an `i32` copy) and branches.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // @solution-start
        let (mut low, mut mid, mut high) = (0, 0, nums.len());
        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => mid += 1,
                _ => {
                    high -= 1;
                    nums.swap(mid, high);
                }
            }
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut nums: Vec<i32>) -> Vec<i32> {
        Solution::sort_colors(&mut nums);
        nums
    }

    #[test]
    fn example_six_values() {
        assert_eq!(sorted(vec![2, 0, 2, 1, 1, 0]), vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_three_values() {
        assert_eq!(sorted(vec![2, 0, 1]), vec![0, 1, 2]);
    }

    #[test]
    fn single_color() {
        assert_eq!(sorted(vec![2, 2, 2]), vec![2, 2, 2]);
        assert_eq!(sorted(vec![0]), vec![0]);
    }

    #[test]
    fn already_sorted_and_reversed() {
        assert_eq!(sorted(vec![0, 1, 2]), vec![0, 1, 2]);
        assert_eq!(sorted(vec![2, 2, 1, 1, 0, 0]), vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn largest_input() {
        let nums: Vec<i32> = (0..300).map(|i| (i * 7) % 3).collect();
        let mut expected = nums.clone();
        expected.sort();
        assert_eq!(sorted(nums), expected);
    }
}
