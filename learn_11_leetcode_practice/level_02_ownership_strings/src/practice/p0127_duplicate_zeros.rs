// ==================================================================
// PRACTICE 0127: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0127_duplicate_zeros.md
// Run the tests:          cargo test practice::p0127
// ==================================================================

//! # Problem 0127: Duplicate Zeros
//!
//! - **Difficulty:** Easy
//! - **Topics:** in-place shifting through `&mut Vec<i32>`, working from the back, virtual indices
//! - **Source:** [LeetCode 1089. Duplicate Zeros](https://leetcode.com/problems/duplicate-zeros/)
//!
//! ## Problem
//!
//! Given a fixed-length vector `arr`, write every `0` twice, shifting the following elements to the
//! right. The length of the vector does not change, so elements pushed beyond the last position are
//! lost. Modify `arr` in place and return nothing.
//!
//! Implement `Solution::duplicate_zeros(arr: &mut Vec<i32>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [1, 0, 2, 3, 0, 4, 5, 0]
//! Output: arr = [1, 0, 0, 2, 3, 0, 0, 4]
//!
//! Input:  arr = [1, 2, 3]
//! Output: arr = [1, 2, 3]
//!
//! Input:  arr = [0, 1, 7, 6, 0]
//! Output: arr = [0, 0, 1, 7, 6]     (the second zero is pushed out)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr.len() <= 10^4`
//! - `0 <= arr[i] <= 9`
//!
//! ## Hints
//!
//! 1. The easy way copies the vector first and rebuilds it. The `O(1)`-space way works from the back.
//! 2. Imagine the longer vector with all zeros duplicated, of length `n + zeros`. Walk the original
//!    from right to left with a write position `j` in that imaginary vector; only write when `j < n`.

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mut arr: Vec<i32>) -> Vec<i32> {
        Solution::duplicate_zeros(&mut arr);
        arr
    }

    /// Simple reference implementation used to check the large case.
    fn expected(arr: &[i32]) -> Vec<i32> {
        let mut out = Vec::with_capacity(arr.len() + 1);
        for &x in arr {
            out.push(x);
            if x == 0 {
                out.push(0);
            }
        }
        out.truncate(arr.len());
        out
    }

    #[test]
    fn example_three_zeros() {
        assert_eq!(
            run(vec![1, 0, 2, 3, 0, 4, 5, 0]),
            vec![1, 0, 0, 2, 3, 0, 0, 4]
        );
    }

    #[test]
    fn example_no_zeros() {
        assert_eq!(run(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn example_last_zero_pushed_out() {
        assert_eq!(run(vec![0, 1, 7, 6, 0]), vec![0, 0, 1, 7, 6]);
    }

    #[test]
    fn zero_at_the_very_end_and_all_zeros() {
        assert_eq!(run(vec![8, 4, 0]), vec![8, 4, 0]);
        assert_eq!(run(vec![0, 0, 0]), vec![0, 0, 0]);
        assert_eq!(run(vec![0]), vec![0]);
    }

    #[test]
    fn large_input() {
        let arr: Vec<i32> = (0..10_000).map(|i| (i * 7) % 10).collect();
        assert_eq!(run(arr.clone()), expected(&arr));
    }
}
