//! # Problem 0043: Shuffle the Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** index arithmetic, `i32` to `usize`, `for` over `0..n`, `push`
//! - **Source:** [LeetCode 1470. Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)
//!
//! ## Problem
//!
//! You are given a vector `nums` of `2n` elements laid out as `[x1, x2, ..., xn, y1, y2, ..., yn]`
//! and the integer `n`. Return the elements interleaved as `[x1, y1, x2, y2, ..., xn, yn]`.
//!
//! Implement `Solution::shuffle(nums: Vec<i32>, n: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 5, 1, 3, 4, 7], n = 3
//! Output: [2, 3, 5, 4, 1, 7]
//! Explanation: x = [2, 5, 1] and y = [3, 4, 7], interleaved.
//!
//! Input:  nums = [1, 1, 2, 2], n = 2
//! Output: [1, 2, 1, 2]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 500`
//! - `nums.len() == 2 * n`
//! - `1 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. `x_i` is at index `i` and `y_i` is at index `i + n` (counting from 0).
//! 2. Convert `n` to `usize` once at the start: `let n = n as usize;`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for each `i` in `0..n`, push `nums[i]` and then `nums[i + n]`.
//!
//! **Complexity:** `O(n)` time, `O(n)` space for the output.
//!
//! **Rust concepts:**
//! - `let n = n as usize;` shadows the `i32` parameter with a `usize` of the same name, so the rest
//!   of the function can use it for indexing.
//! - Index expressions like `nums[i + n]` are computed in `usize`; mixing `usize` and `i32` in one
//!   expression would not compile.
//! - Another approach: `let (xs, ys) = nums.split_at(n);` gives two slices to walk in parallel.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // @solution-start
        let n = n as usize;
        let mut ans = Vec::with_capacity(2 * n);
        for i in 0..n {
            ans.push(nums[i]);
            ans.push(nums[i + n]);
        }
        ans
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_pairs() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn example_two_pairs() {
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn single_pair() {
        assert_eq!(Solution::shuffle(vec![9, 4], 1), vec![9, 4]);
    }

    #[test]
    fn largest_input() {
        // x_i = i + 1 and y_i = i + 501, so the result alternates i + 1, i + 501.
        let nums: Vec<i32> = (1..=1000).collect();
        let ans = Solution::shuffle(nums, 500);
        assert_eq!(ans.len(), 1000);
        assert_eq!(&ans[..4], &[1, 501, 2, 502]);
        assert_eq!(&ans[996..], &[499, 999, 500, 1000]);
    }
}
