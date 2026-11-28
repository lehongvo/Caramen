// ==================================================================
// PRACTICE 0764: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0764_h_index.md
// Run the tests:          cargo test practice::p0764
// ==================================================================

//! # Problem 0764: H-Index
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting sort / buckets, capping values with `min`, reverse scan with `enumerate().rev()`
//! - **Source:** [LeetCode 274. H-Index](https://leetcode.com/problems/h-index/)
//!
//! ## Problem
//!
//! `citations[i]` is the number of citations of a researcher's `i`-th paper. The researcher's
//! **h-index** is the largest `h` such that at least `h` of their papers have **at least** `h`
//! citations each.
//!
//! Implement `Solution::h_index(citations: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  citations = [3, 0, 6, 1, 5]
//! Output: 3             (three papers have >= 3 citations; there are not four with >= 4)
//!
//! Input:  citations = [1, 3, 1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= citations.len() <= 5000`
//! - `0 <= citations[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Sorting in decreasing order works in `O(n log n)`. Can you avoid a comparison sort?
//! 2. The h-index is at most `n`, the number of papers, so any citation count above `n` can be
//!    treated as `n`.
//! 3. Count the papers per (capped) citation value, then scan from `n` down.

pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_papers() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn example_three_papers() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn no_citations() {
        assert_eq!(Solution::h_index(vec![0]), 0);
        assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn one_highly_cited_paper() {
        assert_eq!(Solution::h_index(vec![100]), 1);
    }

    #[test]
    fn h_equals_the_number_of_papers() {
        assert_eq!(Solution::h_index(vec![5, 5, 5, 5, 5]), 5);
    }

    #[test]
    fn large_inputs() {
        assert_eq!(Solution::h_index(vec![1000; 5000]), 1000);
        // Citations 0, 1, ..., 999 repeated 5 times: 5 * (1000 - h) papers have >= h citations.
        let citations: Vec<i32> = (0..5000).map(|i| i % 1000).collect();
        assert_eq!(Solution::h_index(citations), 833);
    }
}
