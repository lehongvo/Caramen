// ==================================================================
// PRACTICE 0306: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0306_count_hills_and_valleys_in_an_array.md
// Run the tests:          cargo test practice::p0306
// ==================================================================

//! # Problem 0306: Count Hills and Valleys in an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec::dedup`, `slice::windows`, `filter` + `count`
//! - **Source:** [LeetCode 2210. Count Hills and Valleys in an Array](https://leetcode.com/problems/count-hills-and-valleys-in-an-array/)
//!
//! ## Problem
//!
//! You are given a vector of integers `nums`. A **hill** is a maximal block of equal, adjacent
//! values whose closest different neighbour on the left **and** closest different neighbour on the
//! right are both smaller. A **valley** is such a block whose two closest different neighbours are
//! both larger. A block needs a different value on **both** sides to count, so the blocks at the
//! very start and end never count. Equal adjacent values belong to the same hill or valley, so a
//! block is counted once no matter how long it is.
//!
//! Return the total number of hills and valleys.
//!
//! Implement `Solution::count_hill_valley(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 4, 1, 1, 6, 5]
//! Output: 3
//! Explanation: 4 is a hill (2 < 4 > 1), the block [1, 1] is a valley (4 > 1 < 6),
//!              and 6 is a hill (1 < 6 > 5).
//!
//! Input:  nums = [6, 6, 5, 5, 4, 1]
//! Output: 0
//! Explanation: the values only go down, so no block has both neighbours smaller or larger.
//! ```
//!
//! ## Constraints
//!
//! - `3 <= nums.len() <= 100`
//! - `1 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. Equal neighbours make the definition awkward. What if you first collapse every run of
//!    equal values into one value?
//! 2. After that, each hill or valley is a single middle element of a window of three values.

pub struct Solution;

impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }

    #[test]
    fn flat_array_has_nothing() {
        assert_eq!(Solution::count_hill_valley(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn long_plateau_counts_once() {
        assert_eq!(Solution::count_hill_valley(vec![1, 5, 5, 5, 5, 1]), 1);
        assert_eq!(Solution::count_hill_valley(vec![5, 5, 1, 1, 1, 5, 5]), 1);
    }

    #[test]
    fn zigzag() {
        // 1 2 1 2 ... : every inner element is a hill or a valley.
        let nums: Vec<i32> = (0..100).map(|i| 1 + i % 2).collect();
        assert_eq!(Solution::count_hill_valley(nums), 98);
    }
}
