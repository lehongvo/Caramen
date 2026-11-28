// ==================================================================
// PRACTICE 0053: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0053_transpose_a_3x3_matrix.md
// Run the tests:          cargo test practice::p0053
// ==================================================================

//! # Problem 0053: Transpose a 3x3 Matrix
//!
//! - **Difficulty:** Easy
//! - **Topics:** arrays `[[i32; 3]; 3]`, arrays are `Copy`, passing arrays by value and by reference, nested loops
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
//!
//! ## Problem
//!
//! A 3x3 matrix is stored as a fixed-size array of 3 rows, each an array of 3 integers:
//! `[[i32; 3]; 3]`. Write three functions:
//!
//! 1. `transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3]` returns the transpose: the element at row `r`,
//!    column `c` moves to row `c`, column `r`.
//! 2. `trace(m: &[[i32; 3]; 3]) -> i32` returns the sum of the main diagonal `m[0][0] + m[1][1] + m[2][2]`.
//! 3. `is_symmetric(m: &[[i32; 3]; 3]) -> bool` returns `true` when the matrix equals its transpose.
//!
//! ## Examples
//!
//! ```text
//! m = [[1, 2, 3],
//!      [4, 5, 6],
//!      [7, 8, 9]]
//!
//! transpose(m)       -> [[1, 4, 7],
//!                        [2, 5, 8],
//!                        [3, 6, 9]]
//! trace(&m)          -> 15
//! is_symmetric(&m)   -> false
//!
//! is_symmetric(&[[1, 7, 3], [7, 4, 5], [3, 5, 6]]) -> true
//! ```
//!
//! ## Constraints
//!
//! - Values are between `-1000` and `1000`.
//!
//! ## Hints
//!
//! 1. Start from `let mut t = [[0; 3]; 3];` and fill `t[c][r] = m[r][c]`.
//! 2. Arrays implement `==`, so `transpose(*m) == *m` compares all nine elements.

pub fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!("Write your solution here")
}

pub fn trace(m: &[[i32; 3]; 3]) -> i32 {
    todo!("Write your solution here")
}

pub fn is_symmetric(m: &[[i32; 3]; 3]) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    #[test]
    fn transpose_example() {
        assert_eq!(transpose(M), [[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
    }

    #[test]
    fn transpose_twice_is_identity_and_original_is_unchanged() {
        let m = [[-1, 0, 1000], [5, -1000, 2], [3, 3, 3]];
        let t = transpose(m);
        assert_eq!(transpose(t), m);
        // `m` was copied into `transpose`, so it is still usable here.
        assert_eq!(m[0][2], 1000);
    }

    #[test]
    fn trace_examples() {
        assert_eq!(trace(&M), 15);
        assert_eq!(trace(&[[0; 3]; 3]), 0);
        assert_eq!(trace(&[[-1000, 1, 1], [1, -1000, 1], [1, 1, -1000]]), -3000);
    }

    #[test]
    fn symmetry() {
        assert!(!is_symmetric(&M));
        assert!(is_symmetric(&[[1, 7, 3], [7, 4, 5], [3, 5, 6]]));
        assert!(is_symmetric(&[[0; 3]; 3]));
        assert!(!is_symmetric(&[[1, 7, 3], [7, 4, 5], [3, 6, 6]]));
    }
}
