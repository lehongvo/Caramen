// ==================================================================
// PRACTICE 0749: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0749_sum_of_subarray_minimums.md
// Run the tests:          cargo test practice::p0749
// ==================================================================

//! # Problem 0749: Sum of Subarray Minimums
//!
//! - **Difficulty:** Medium
//! - **Topics:** previous / next smaller element with monotonic stacks, contribution counting, modular sums
//! - **Source:** [LeetCode 907. Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums/)
//!
//! ## Problem
//!
//! Given a vector `arr`, consider every non-empty contiguous subarray and take its minimum.
//! Return the sum of all these minimums. The sum can be huge, so return it modulo
//! `1_000_000_007`.
//!
//! Implement `Solution::sum_subarray_mins(arr: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [3, 1, 2, 4]
//! Output: 17
//!         (subarrays: [3] [1] [2] [4] [3,1] [1,2] [2,4] [3,1,2] [1,2,4] [3,1,2,4]
//!          minimums:   3   1   2   4    1     1     2      1       1        1   -> sum 17)
//!
//! Input:  arr = [11, 81, 94, 43, 3]
//! Output: 444
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr.len() <= 3 * 10^4`
//! - `1 <= arr[i] <= 3 * 10^4`
//!
//! ## Hints
//!
//! 1. Turn the question around: for each element, in how many subarrays is it the minimum?
//! 2. That count is (choices for the left end) times (choices for the right end), limited by the
//!    nearest smaller elements on each side. Monotonic stacks find them in `O(n)`.
//! 3. With equal values, make sure each subarray is credited to exactly one of them: use "strictly
//!    smaller" on one side and "smaller or equal" on the other.

pub struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_values() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn example_five_values() {
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }

    #[test]
    fn single_value() {
        assert_eq!(Solution::sum_subarray_mins(vec![1]), 1);
    }

    #[test]
    fn equal_values_are_counted_once() {
        // Six subarrays, each with minimum 2.
        assert_eq!(Solution::sum_subarray_mins(vec![2, 2, 2]), 12);
    }

    #[test]
    fn increasing_values() {
        // 1 * 4 + 2 * 3 + 3 * 2 + 4 * 1
        assert_eq!(Solution::sum_subarray_mins(vec![1, 2, 3, 4]), 20);
    }

    #[test]
    fn large_input_needs_the_modulo() {
        // 30_000 * (30_000 * 30_001 / 2) = 13_500_450_000_000, modulo 1_000_000_007.
        assert_eq!(
            Solution::sum_subarray_mins(vec![30_000; 30_000]),
            449_905_500
        );
    }
}
