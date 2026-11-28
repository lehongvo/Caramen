//! # Problem 0325: Pascal's Triangle
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec<Vec<i32>>`, borrowing the previous row, `windows(2)`, `extend`
//! - **Source:** [LeetCode 118. Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)
//!
//! ## Problem
//!
//! Return the first `num_rows` rows of Pascal's triangle. Row `0` is `[1]`. Every following row
//! starts and ends with `1`, and each value in between is the sum of the two values directly above
//! it (the values at positions `j - 1` and `j` of the previous row).
//!
//! Implement `Solution::generate(num_rows: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num_rows = 5
//! Output: [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
//!
//! Input:  num_rows = 1
//! Output: [[1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num_rows <= 30`
//!
//! ## Hints
//!
//! 1. Build the rows one after another; `&rows[i - 1]` is the previous one.
//! 2. The inner values of a new row are the sums of the neighbouring pairs `windows(2)` of the
//!    previous row.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** start with `[[1]]`. For each new row, take the previous row `prev`, and build
//! `[1] + [a + b for each window [a, b] of prev] + [1]`.
//!
//! **Complexity:** `O(num_rows^2)` time and space (the output itself has that many numbers).
//!
//! **Rust concepts:**
//! - `let prev = &rows[i - 1];` borrows the previous row. The borrow checker accepts
//!   `rows.push(row)` afterwards because `prev` is not used after `row` is built (non-lexical
//!   lifetimes); pushing while `prev` is still needed would be rejected, since a push may move
//!   the whole buffer.
//! - `Vec::with_capacity(n)` avoids re-allocations when the final size is known.
//! - `extend(prev.windows(2).map(|w| w[0] + w[1]))` appends every computed value at once.
//! - The largest value in row 29 is `C(29, 14) = 77_558_760`, which fits in `i32`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let n = num_rows as usize;
        let mut rows: Vec<Vec<i32>> = Vec::with_capacity(n);
        rows.push(vec![1]);
        for i in 1..n {
            let prev = &rows[i - 1];
            let mut row = Vec::with_capacity(prev.len() + 1);
            row.push(1);
            row.extend(prev.windows(2).map(|w| w[0] + w[1]));
            row.push(1);
            // `prev` is no longer used here, so `rows` can be borrowed mutably again.
            rows.push(row);
        }
        rows
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_rows() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn example_one_row() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn two_rows() {
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    }

    #[test]
    fn thirty_rows() {
        let rows = Solution::generate(30);
        assert_eq!(rows.len(), 30);
        assert_eq!(rows[29].len(), 30);
        assert_eq!(rows[29][14], 77_558_760);
        // Row i sums to 2^i.
        assert_eq!(rows[29].iter().map(|&x| x as i64).sum::<i64>(), 1 << 29);
    }
}
