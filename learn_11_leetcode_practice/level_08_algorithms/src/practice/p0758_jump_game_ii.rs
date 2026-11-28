// ==================================================================
// PRACTICE 0758: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0758_jump_game_ii.md
// Run the tests:          cargo test practice::p0758
// ==================================================================

//! # Problem 0758: Jump Game II
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy BFS by levels, window `current_end`, `take(n - 1)`
//! - **Source:** [LeetCode 45. Jump Game II](https://leetcode.com/problems/jump-game-ii/)
//!
//! ## Problem
//!
//! You start at index `0` of a vector `nums`; from index `i` you may jump forward by `0` to
//! `nums[i]` positions. The input guarantees that the last index is reachable. Return the
//! **minimum** number of jumps to reach the last index.
//!
//! Implement `Solution::jump(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 3, 1, 1, 4]
//! Output: 2             (0 -> 1 -> 4)
//!
//! Input:  nums = [2, 3, 0, 1, 4]
//! Output: 2
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `0 <= nums[i] <= 1000`
//! - The last index is reachable.
//!
//! ## Hints
//!
//! 1. Group the indices by the minimum number of jumps needed to reach them. Each group is a
//!    contiguous range.
//! 2. While scanning the range reachable with `k` jumps, compute the farthest index reachable with
//!    `k + 1` jumps.

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_jumps() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example_with_zero() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn already_at_the_end() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn one_step_at_a_time() {
        assert_eq!(Solution::jump(vec![1, 1, 1, 1]), 3);
        assert_eq!(Solution::jump(vec![1, 2]), 1);
    }

    #[test]
    fn landing_as_far_as_possible_is_not_best() {
        // Landing on index 3 first needs 4 jumps in total; landing on index 1 (value 4) needs 3.
        assert_eq!(Solution::jump(vec![3, 4, 1, 1, 1, 1, 1]), 3);
    }

    #[test]
    fn long_input() {
        assert_eq!(Solution::jump(vec![1; 10_000]), 9_999);
        assert_eq!(Solution::jump(vec![1000; 10_000]), 10);
    }
}
