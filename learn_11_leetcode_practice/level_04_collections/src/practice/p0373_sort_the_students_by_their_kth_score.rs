// ==================================================================
// PRACTICE 0373: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0373_sort_the_students_by_their_kth_score.md
// Run the tests:          cargo test practice::p0373
// ==================================================================

//! # Problem 0373: Sort the Students by Their Kth Score
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting the rows of a `Vec<Vec<i32>>` by one column, `sort_unstable_by_key` with `Reverse`, moving rows instead of copying
//! - **Source:** [LeetCode 2545. Sort the Students by Their Kth Score](https://leetcode.com/problems/sort-the-students-by-their-kth-score/)
//!
//! ## Problem
//!
//! A class has `m` students and `n` exams. `score[i][j]` is the score of student `i` on exam `j`;
//! all scores in the matrix are **distinct**. Given an exam index `k`, return the matrix with its
//! rows (students) reordered from the **highest** to the **lowest** score on exam `k`. Each row is
//! kept intact; only the order of the rows changes.
//!
//! Implement `Solution::sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  score = [[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]], k = 2
//! Output: [[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]]
//! Explanation: exam 2 scores are 9, 11, 3, so the order is 11, 9, 3.
//!
//! Input:  score = [[3, 4], [5, 6]], k = 0
//! Output: [[5, 6], [3, 4]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 250`
//! - `1 <= score[i][j] <= 10^5`, all distinct
//! - `0 <= k < n`
//!
//! ## Hints
//!
//! 1. Sorting a `Vec<Vec<i32>>` moves the inner vectors around; it never copies their contents.
//! 2. The key of a row is `row[k]`; sort by `Reverse(row[k])` for descending order.

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            ),
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::sort_the_students(vec![vec![3, 4], vec![5, 6]], 0),
            vec![vec![5, 6], vec![3, 4]]
        );
    }

    #[test]
    fn single_student_and_last_column() {
        assert_eq!(
            Solution::sort_the_students(vec![vec![1, 2, 3]], 2),
            vec![vec![1, 2, 3]]
        );
        assert_eq!(
            Solution::sort_the_students(vec![vec![1, 9], vec![2, 8], vec![3, 10]], 1),
            vec![vec![3, 10], vec![1, 9], vec![2, 8]]
        );
    }

    #[test]
    fn largest_matrix() {
        // Distinct scores: row r, column c holds r * 250 + c + 1 (row 249 is the best everywhere).
        let score: Vec<Vec<i32>> = (0..250)
            .map(|r| (0..250).map(|c| r * 250 + c + 1).collect())
            .collect();
        let sorted = Solution::sort_the_students(score.clone(), 100);
        let expected: Vec<Vec<i32>> = score.into_iter().rev().collect();
        assert_eq!(sorted, expected);
    }
}
