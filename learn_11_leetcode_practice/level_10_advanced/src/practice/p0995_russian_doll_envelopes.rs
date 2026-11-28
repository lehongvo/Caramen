// ==================================================================
// PRACTICE 0995: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0995_russian_doll_envelopes.md
// Run the tests:          cargo test practice::p0995
// ==================================================================

//! # Problem 0995: Russian Doll Envelopes
//!
//! - **Difficulty:** Hard
//! - **Topics:** sorting by `(width asc, height desc)` with `sort_unstable_by`, LIS with patience sorting and `partition_point`, `Ordering::then`
//! - **Source:** [LeetCode 354. Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes/)
//!
//! ## Problem
//!
//! `envelopes[i] = [w, h]`. One envelope fits into another if **both** its width and its height are
//! strictly smaller (no rotation). Return the maximum number of envelopes you can nest one inside
//! another.
//!
//! Implement `Solution::max_envelopes(envelopes: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! envelopes = [[5, 4], [6, 4], [6, 7], [2, 3]]   => 3      ([2,3] -> [5,4] -> [6,7])
//! envelopes = [[1, 1], [1, 1], [1, 1]]           => 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= envelopes.len() <= 10^5`, `1 <= w, h <= 10^5`
//!
//! ## Hints
//!
//! 1. Sort by width; then the answer is a strictly increasing subsequence of heights.
//! 2. Envelopes with the **same** width cannot nest: sort them by **decreasing** height, so at most
//!    one of them can be part of an increasing sequence.
//! 3. Compute the longest strictly increasing subsequence in `O(n log n)`: `tails[k]` is the smallest
//!    possible last height of an increasing sequence of length `k + 1`.

pub struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(envelopes: &[[i32; 2]]) -> i32 {
        Solution::max_envelopes(envelopes.iter().map(|e| e.to_vec()).collect())
    }

    fn naive(envelopes: &[[i32; 2]]) -> i32 {
        let mut sorted = envelopes.to_vec();
        sorted.sort_unstable();
        let mut best = vec![1; sorted.len()];
        for i in 0..sorted.len() {
            for j in 0..i {
                if sorted[j][0] < sorted[i][0] && sorted[j][1] < sorted[i][1] {
                    best[i] = best[i].max(best[j] + 1);
                }
            }
        }
        best.into_iter().max().unwrap_or(0)
    }

    #[test]
    fn examples() {
        assert_eq!(run(&[[5, 4], [6, 4], [6, 7], [2, 3]]), 3);
        assert_eq!(run(&[[1, 1], [1, 1], [1, 1]]), 1);
    }

    #[test]
    fn equal_widths_do_not_nest() {
        assert_eq!(run(&[[1, 1], [1, 2], [1, 3]]), 1);
        assert_eq!(run(&[[4, 5], [4, 6], [6, 7], [2, 3], [1, 1]]), 4);
        assert_eq!(run(&[[1, 3], [3, 5], [6, 7], [6, 8], [8, 4], [9, 5]]), 3);
    }

    #[test]
    fn matches_quadratic_dp() {
        let envelopes: Vec<[i32; 2]> = (0..400)
            .map(|i: i32| [(i * 37) % 50 + 1, (i * 53) % 60 + 1])
            .collect();
        assert_eq!(run(&envelopes), naive(&envelopes));
    }

    #[test]
    fn large_chain() {
        let envelopes: Vec<[i32; 2]> = (1..=100_000).rev().map(|i| [i, i]).collect();
        assert_eq!(run(&envelopes), 100_000);
    }
}
