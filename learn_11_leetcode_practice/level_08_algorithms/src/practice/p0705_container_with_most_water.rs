// ==================================================================
// PRACTICE 0705: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0705_container_with_most_water.md
// Run the tests:          cargo test practice::p0705
// ==================================================================

//! # Problem 0705: Container With Most Water
//!
//! - **Difficulty:** Medium
//! - **Topics:** two pointers, greedy elimination argument, `max`
//! - **Source:** [LeetCode 11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/)
//!
//! ## Problem
//!
//! `height[i]` is the height of a vertical line drawn at position `i` on the x-axis. Choose two of
//! the lines; together with the x-axis they form a container. The water it holds is
//! `(j - i) * min(height[i], height[j])` for lines `i < j` (the container cannot be tilted).
//!
//! Return the largest amount of water any pair of lines can hold.
//!
//! Implement `Solution::max_area(height: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
//! Output: 49            (lines 1 and 8: width 7, height min(8, 7) = 7)
//!
//! Input:  height = [1, 1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `2 <= height.len() <= 10^5`
//! - `0 <= height[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Trying every pair is `O(n^2)`, too slow for `10^5` lines.
//! 2. Start with the widest container (the two outer lines). Which line can you safely discard?

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_classic() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_two_lines() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn outer_lines_win() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn zero_height_lines() {
        assert_eq!(Solution::max_area(vec![0, 0, 0]), 0);
    }

    #[test]
    fn best_pair_is_inside() {
        // Lines at 1 and 3 (height 10, width 2) beat the outer pair (height 1, width 4).
        assert_eq!(Solution::max_area(vec![1, 10, 1, 10, 1]), 20);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::max_area(vec![10_000; 100_000]), 10_000 * 99_999);
    }
}
