// ==================================================================
// PRACTICE 0985: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0985_n_queens_ii.md
// Run the tests:          cargo test practice::p0985
// ==================================================================

//! # Problem 0985: N-Queens II
//!
//! - **Difficulty:** Hard
//! - **Topics:** bitmask backtracking, `u32` masks shifted per row, `trailing_zeros` and `x & x.wrapping_neg()` to pick bits
//! - **Source:** [LeetCode 52. N-Queens II](https://leetcode.com/problems/n-queens-ii/)
//!
//! ## Problem
//!
//! Count the distinct ways to place `n` non-attacking queens on an `n x n` board (see problem 0984),
//! without building the boards.
//!
//! Implement `Solution::total_n_queens(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4      Output: 2
//! Input:  n = 1      Output: 1
//! Input:  n = 8      Output: 92
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 9`
//!
//! ## Hints
//!
//! 1. Represent the attacked columns of the current row as three bitmasks: columns, "left"
//!    diagonals and "right" diagonals.
//! 2. Moving to the next row shifts the diagonal masks by one bit (left and right).
//! 3. The free squares are `!(cols | left | right) & full`; pop them one at a time with
//!    `bit = free & free.wrapping_neg()`.

pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(8), 92);
    }

    #[test]
    fn no_solutions() {
        assert_eq!(Solution::total_n_queens(2), 0);
        assert_eq!(Solution::total_n_queens(3), 0);
    }

    #[test]
    fn known_counts() {
        let counts: Vec<i32> = (1..=9).map(Solution::total_n_queens).collect();
        assert_eq!(counts, vec![1, 0, 0, 2, 10, 4, 40, 92, 352]);
    }

    #[test]
    fn beyond_the_constraints() {
        assert_eq!(Solution::total_n_queens(10), 724);
        assert_eq!(Solution::total_n_queens(11), 2_680);
    }
}
