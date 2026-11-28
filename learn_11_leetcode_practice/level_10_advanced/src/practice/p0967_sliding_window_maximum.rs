// ==================================================================
// PRACTICE 0967: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0967_sliding_window_maximum.md
// Run the tests:          cargo test practice::p0967
// ==================================================================

//! # Problem 0967: Sliding Window Maximum
//!
//! - **Difficulty:** Hard
//! - **Topics:** monotonic `VecDeque` of indices, `front`/`back`/`pop_front`/`pop_back`, amortised analysis
//! - **Source:** [LeetCode 239. Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)
//!
//! ## Problem
//!
//! A window of size `k` slides over `nums` from left to right, one position at a time. Return the
//! maximum of each window position, in order (`nums.len() - k + 1` values).
//!
//! Implement `Solution::max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 3, -1, -3, 5, 3, 6, 7], k = 3
//! Output: [3, 3, 5, 5, 6, 7]
//!
//! Input:  nums = [1], k = 1
//! Output: [1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`
//! - `1 <= k <= nums.len()`
//!
//! ## Hints
//!
//! 1. Keep a deque of indices whose values are **decreasing** from front to back.
//! 2. Before pushing index `i`, pop from the back every index whose value is `<= nums[i]`: those can
//!    never be a maximum again while `i` is in the window.
//! 3. Pop the front when it falls out of the window; the front is always the current maximum.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn window_sizes_one_and_n() {
        assert_eq!(
            Solution::max_sliding_window(vec![4, -2, 9], 1),
            vec![4, -2, 9]
        );
        assert_eq!(Solution::max_sliding_window(vec![4, -2, 9, 1], 4), vec![9]);
        assert_eq!(
            Solution::max_sliding_window(vec![2, 2, 2, 2], 2),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..500).map(|i: i32| (i * 7_919) % 201 - 100).collect();
        for k in [1, 2, 7, 50, 500] {
            let expected: Vec<i32> = nums.windows(k).map(|w| *w.iter().max().unwrap()).collect();
            assert_eq!(
                Solution::max_sliding_window(nums.clone(), k as i32),
                expected
            );
        }
    }

    #[test]
    fn large_decreasing_input() {
        let nums: Vec<i32> = (0..100_000).map(|i| 10_000 - i % 20_001).collect();
        let result = Solution::max_sliding_window(nums.clone(), 1_000);
        assert_eq!(result.len(), 99_001);
        assert_eq!(result[0], nums[0]);
    }
}
