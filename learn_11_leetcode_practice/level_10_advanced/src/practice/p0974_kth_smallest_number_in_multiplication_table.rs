// ==================================================================
// PRACTICE 0974: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0974_kth_smallest_number_in_multiplication_table.md
// Run the tests:          cargo test practice::p0974
// ==================================================================

//! # Problem 0974: Kth Smallest Number in Multiplication Table
//!
//! - **Difficulty:** Hard
//! - **Topics:** binary search on values, counting elements `<= x` row by row, `min`, implicit matrices
//! - **Source:** [LeetCode 668. Kth Smallest Number in Multiplication Table](https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/)
//!
//! ## Problem
//!
//! The `m x n` multiplication table has the value `i * j` at row `i`, column `j` (both 1-based).
//! Return the `k`-th smallest element of the table, counting duplicates.
//!
//! Implement `Solution::find_kth_number(m: i32, n: i32, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  m = 3, n = 3, k = 5
//! Output: 3            (sorted: 1, 2, 2, 3, 3, 4, 6, 6, 9)
//!
//! Input:  m = 2, n = 3, k = 6
//! Output: 6            (sorted: 1, 2, 2, 3, 4, 6)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 3 * 10^4`
//! - `1 <= k <= m * n`
//!
//! ## Hints
//!
//! 1. The table can have `9 * 10^8` entries: never build it.
//! 2. Row `i` contains `min(x / i, n)` values `<= x`, so counting all values `<= x` takes `O(m)`.
//! 3. Binary search the smallest `x` whose count is at least `k`; that `x` is always in the table.

pub struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(m: i32, n: i32, k: i32) -> i32 {
        let mut all: Vec<i32> = (1..=m).flat_map(|i| (1..=n).map(move |j| i * j)).collect();
        all.sort_unstable();
        all[k as usize - 1]
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }

    #[test]
    fn single_row_and_corners() {
        assert_eq!(Solution::find_kth_number(1, 10, 7), 7);
        assert_eq!(Solution::find_kth_number(10, 1, 10), 10);
        assert_eq!(Solution::find_kth_number(5, 5, 1), 1);
        assert_eq!(Solution::find_kth_number(5, 5, 25), 25);
    }

    #[test]
    fn matches_sorting() {
        for (m, n) in [(4, 7), (9, 3), (12, 12)] {
            for k in 1..=m * n {
                assert_eq!(
                    Solution::find_kth_number(m, n, k),
                    naive(m, n, k),
                    "{m}x{n}, k={k}"
                );
            }
        }
    }

    #[test]
    fn largest_table() {
        assert_eq!(
            Solution::find_kth_number(30_000, 30_000, 900_000_000),
            900_000_000
        );
        assert_eq!(Solution::find_kth_number(30_000, 30_000, 1), 1);
    }
}
