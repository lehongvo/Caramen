//! # Problem 0458: Find the Peaks
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::windows(3)`, `enumerate`, `filter_map`, index arithmetic on windows
//! - **Source:** [LeetCode 2951. Find the Peaks](https://leetcode.com/problems/find-the-peaks/)
//!
//! ## Problem
//!
//! `mountain[i]` is the height at position `i`. A *peak* is a position that is **strictly** higher
//! than both of its neighbours. The first and the last positions are never peaks (they have only
//! one neighbour).
//!
//! Return the indexes of all peaks in increasing order.
//!
//! Implement `Solution::find_peaks(mountain: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mountain = [2, 4, 4]
//! Output: []
//! Explanation: index 1 is not strictly higher than index 2.
//!
//! Input:  mountain = [1, 4, 3, 8, 5]
//! Output: [1, 3]
//! ```
//!
//! ## Constraints
//!
//! - `3 <= mountain.len() <= 100`
//! - `1 <= mountain[i] <= 100`
//!
//! ## Hints
//!
//! 1. `mountain.windows(3)` yields every group `[left, middle, right]` of three neighbours.
//! 2. The window with index `i` (from `enumerate`) has its middle element at index `i + 1`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** slide a window of three over the heights; a window whose middle is strictly larger
//! than both sides marks a peak at "window index + 1".
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space besides the output.
//!
//! **Rust concepts:**
//! - `windows(k)` yields overlapping sub-slices of length `k` without copying; there are
//!   `len - k + 1` of them, so the first and last positions are naturally never a middle.
//! - Slice patterns in the closure, `&[a, b, c]`, would not compile here because `windows` yields
//!   `&[i32]` of unknown length; indexing `w[0]`, `w[1]`, `w[2]` is safe since every window has
//!   exactly 3 elements.
//! - `filter_map` with `bool::then_some` combines the test and the index computation.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        // @solution-start
        mountain
            .windows(3)
            .enumerate()
            .filter_map(|(i, w)| (w[1] > w[0] && w[1] > w[2]).then_some(i as i32 + 1))
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_plateau() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), Vec::<i32>::new());
    }

    #[test]
    fn example_two_peaks() {
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }

    #[test]
    fn edges_are_not_peaks() {
        assert_eq!(Solution::find_peaks(vec![9, 1, 9]), Vec::<i32>::new());
        assert_eq!(Solution::find_peaks(vec![1, 2, 1]), vec![1]);
    }

    #[test]
    fn zigzag() {
        let mountain: Vec<i32> = (0..100).map(|i| if i % 2 == 0 { 1 } else { 100 }).collect();
        let expected: Vec<i32> = (1..99).step_by(2).collect();
        assert_eq!(Solution::find_peaks(mountain), expected);
    }
}
