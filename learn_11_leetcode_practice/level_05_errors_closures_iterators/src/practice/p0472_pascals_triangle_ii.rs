// ==================================================================
// PRACTICE 0472: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0472_pascals_triangle_ii.md
// Run the tests:          cargo test practice::p0472
// ==================================================================

//! # Problem 0472: Pascal's Triangle II
//!
//! - **Difficulty:** Easy
//! - **Topics:** `iter::successors` to generate rows, `windows(2)`, `iter::once` + `chain`, `nth`, `checked_add` collected into `Option<Vec<_>>`
//! - **Source:** [LeetCode 119. Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)
//!
//! ## Problem
//!
//! In Pascal's triangle, row `0` is `[1]`, and every next row starts and ends with `1` while each
//! inner value is the sum of the two values just above it:
//!
//! ```text
//! row 0:        1
//! row 1:       1 1
//! row 2:      1 2 1
//! row 3:     1 3 3 1
//! row 4:    1 4 6 4 1
//! ```
//!
//! Given `row_index`, return that row (0-based).
//!
//! Implement `Solution::get_row(row_index: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  row_index = 3
//! Output: [1, 3, 3, 1]
//!
//! Input:  row_index = 0
//! Output: [1]
//!
//! Input:  row_index = 1
//! Output: [1, 1]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= row_index <= 33` (every value fits in an `i32`).
//!
//! ## Hints
//!
//! 1. Next row = `1`, then the sums of neighbouring pairs of the previous row, then `1`.
//!    `prev.windows(2).map(|w| w[0] + w[1])` gives the sums.
//! 2. `iter::successors(Some(first), |prev| next_of(prev))` is an iterator of rows; `nth(k)` picks
//!    row `k`.
//! 3. Careful: `successors` computes the **next** item at the moment it returns the current one, so
//!    asking for row 33 also builds row 34, whose middle values do not fit in an `i32`. Use
//!    `checked_add` and let the closure return `None` on overflow.

use std::iter;

pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_row_three() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn example_first_rows() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }

    #[test]
    fn largest_row() {
        let row = Solution::get_row(33);
        assert_eq!(row.len(), 34);
        assert_eq!(row[16], 1_166_803_110);
        assert_eq!(row[1], 33);
        assert_eq!(row.iter().map(|&x| x as i64).sum::<i64>(), 1i64 << 33);
    }

    #[test]
    fn rows_are_symmetric() {
        for k in 0..=33 {
            let row = Solution::get_row(k);
            assert!(row.iter().eq(row.iter().rev()));
        }
    }
}
