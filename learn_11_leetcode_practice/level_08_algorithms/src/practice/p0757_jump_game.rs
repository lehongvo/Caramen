// ==================================================================
// PRACTICE 0757: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0757_jump_game.md
// Run the tests:          cargo test practice::p0757
// ==================================================================

//! # Problem 0757: Jump Game
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy farthest reach, early return
//! - **Source:** [LeetCode 55. Jump Game](https://leetcode.com/problems/jump-game/)
//!
//! ## Problem
//!
//! You start at index `0` of a vector `nums`. From index `i` you may jump forward by any number of
//! positions from `0` to `nums[i]`. Return `true` if you can reach the last index, `false`
//! otherwise.
//!
//! Implement `Solution::can_jump(nums: Vec<i32>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 3, 1, 1, 4]
//! Output: true          (0 -> 1 -> 4)
//!
//! Input:  nums = [3, 2, 1, 0, 4]
//! Output: false         (every path gets stuck on index 3, whose value is 0)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `0 <= nums[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. You do not need the actual path. Track the farthest index reachable so far.
//! 2. If the current index is beyond that farthest index, you are stuck.

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_reachable() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_stuck_on_zero() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn single_index_is_already_the_end() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn zero_at_the_start() {
        assert!(!Solution::can_jump(vec![0, 1]));
    }

    #[test]
    fn zero_at_the_end_is_fine() {
        assert!(Solution::can_jump(vec![1, 1, 0]));
    }

    #[test]
    fn long_inputs() {
        assert!(Solution::can_jump(vec![1; 10_000]));
        let mut one_big_jump = vec![0; 10_000];
        one_big_jump[0] = 9_999;
        assert!(Solution::can_jump(one_big_jump));
        let mut gap = vec![1; 10_000];
        gap[5_000] = 0;
        assert!(!Solution::can_jump(gap));
    }
}
