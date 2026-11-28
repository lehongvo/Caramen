// ==================================================================
// PRACTICE 0965: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0965_largest_rectangle_in_histogram.md
// Run the tests:          cargo test practice::p0965
// ==================================================================

//! # Problem 0965: Largest Rectangle in Histogram
//!
//! - **Difficulty:** Hard
//! - **Topics:** monotonic stack of indices, a sentinel bar of height 0, `chain(once(0))`, widths from neighbouring stack entries
//! - **Source:** [LeetCode 84. Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)
//!
//! ## Problem
//!
//! `heights[i]` is the height of bar `i` in a histogram whose bars all have width 1. Return the
//! area of the largest axis-aligned rectangle that fits entirely inside the histogram.
//!
//! Implement `Solution::largest_rectangle_area(heights: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  heights = [2, 1, 5, 6, 2, 3]
//! Output: 10         (height 5 over bars 2..=3)
//!
//! Input:  heights = [2, 4]
//! Output: 4
//! ```
//!
//! ## Constraints
//!
//! - `1 <= heights.len() <= 10^5`
//! - `0 <= heights[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. The best rectangle has the height of some bar `i` and extends left and right until a strictly
//!    lower bar.
//! 2. Keep a stack of indices with increasing heights. When a lower bar arrives, every taller bar on
//!    the stack has found its right limit; its left limit is the entry below it on the stack.
//! 3. Append a height-0 bar at the end so that everything is popped.

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(heights: &[i32]) -> i32 {
        let mut best = 0;
        for i in 0..heights.len() {
            let mut low = i32::MAX;
            for (j, &h) in heights.iter().enumerate().skip(i) {
                low = low.min(h);
                best = best.max(low * (j - i + 1) as i32);
            }
        }
        best
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn flat_zero_and_single() {
        assert_eq!(Solution::largest_rectangle_area(vec![3, 3, 3]), 9);
        assert_eq!(Solution::largest_rectangle_area(vec![0, 0]), 0);
        assert_eq!(Solution::largest_rectangle_area(vec![7]), 7);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 0, 2]), 2);
    }

    #[test]
    fn matches_naive() {
        let heights: Vec<i32> = (0..300).map(|i: i32| (i * 7_919) % 23).collect();
        assert_eq!(
            Solution::largest_rectangle_area(heights.clone()),
            naive(&heights)
        );
    }

    #[test]
    fn large_inputs() {
        assert_eq!(
            Solution::largest_rectangle_area(vec![10_000; 100_000]),
            1_000_000_000
        );
        let rising: Vec<i32> = (1..=10_000).collect();
        // Best is height k over the 10_001 - k bars >= k: maximised at k = 5_000 or 5_001.
        assert_eq!(Solution::largest_rectangle_area(rising), 5_000 * 5_001);
    }
}
