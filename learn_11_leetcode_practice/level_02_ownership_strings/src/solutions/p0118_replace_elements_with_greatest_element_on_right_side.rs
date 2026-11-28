//! # Problem 0118: Replace Elements with Greatest Element on Right Side
//!
//! - **Difficulty:** Easy
//! - **Topics:** reusing an owned `Vec`, `iter_mut().rev()`, a running maximum
//! - **Source:** [LeetCode 1299. Replace Elements with Greatest Element on Right Side](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/)
//!
//! ## Problem
//!
//! Given a vector `arr`, replace every element with the largest value among the elements **to its
//! right** (strictly after it). The last element has nothing to its right, so it becomes `-1`.
//!
//! Implement `Solution::replace_elements(arr: Vec<i32>) -> Vec<i32>` and return the modified
//! vector. Since the function owns `arr`, you can overwrite it in place and return it.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [17, 18, 5, 4, 6, 1]
//! Output: [18, 6, 6, 6, 1, -1]
//! Explanation: right of 17 the maximum is 18; right of 18 it is 6; ...; nothing is right of 1.
//!
//! Input:  arr = [400]
//! Output: [-1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr.len() <= 10^4`
//! - `1 <= arr[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. Computing a maximum of the right part for every index is `O(n^2)`. Process the vector from
//!    the back instead and keep "the maximum seen so far".
//! 2. Save the current value before overwriting it: you still need it to update the maximum.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** walk from right to left with a running maximum `best` that starts at `-1`. For every
//! element, remember its old value, overwrite it with `best`, then update `best` with the old value.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space (the input vector is reused).
//!
//! **Rust concepts:**
//! - `mut arr: Vec<i32>` turns the owned parameter into a buffer we may overwrite and return.
//! - `iter_mut().rev()` gives `&mut i32` from the back; `let old = *x;` copies the `i32` out
//!   (cheap, because `i32` is `Copy`) before `*x = best` overwrites it.
//! - `i32::max` returns the larger of two values.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut best = -1;
        for x in arr.iter_mut().rev() {
            let old = *x;
            *x = best;
            best = best.max(old);
        }
        arr
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
    }

    #[test]
    fn increasing_values() {
        assert_eq!(
            Solution::replace_elements(vec![1, 2, 3, 4]),
            vec![4, 4, 4, -1]
        );
    }

    #[test]
    fn decreasing_values() {
        assert_eq!(
            Solution::replace_elements(vec![4, 3, 2, 1]),
            vec![3, 2, 1, -1]
        );
    }

    #[test]
    fn large_input() {
        let arr: Vec<i32> = (1..=10_000).collect();
        let result = Solution::replace_elements(arr);
        assert!(result[..9_999].iter().all(|&v| v == 10_000));
        assert_eq!(result[9_999], -1);
    }
}
