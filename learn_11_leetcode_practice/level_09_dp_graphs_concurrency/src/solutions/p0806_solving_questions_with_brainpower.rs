//! # Problem 0806: Solving Questions With Brainpower
//!
//! - **Difficulty:** Medium
//! - **Topics:** backward (suffix) dynamic programming, `i64` results, `Vec::get` for out-of-range
//! - **Source:** [LeetCode 2140. Solving Questions With Brainpower](https://leetcode.com/problems/solving-questions-with-brainpower/)
//!
//! ## Problem
//!
//! An exam has questions `0..n` that must be handled **in order**. `questions[i] = [points,
//! brainpower]`. For each question you either:
//!
//! - **solve** it: you earn `points`, but you are too tired for the next `brainpower` questions,
//!   which are skipped automatically; or
//! - **skip** it and move on to question `i + 1`.
//!
//! Return the maximum number of points you can earn. The result can exceed `i32`, so the function
//! returns an `i64`.
//!
//! Implement `Solution::most_points(questions: Vec<Vec<i32>>) -> i64`.
//!
//! ## Examples
//!
//! ```text
//! Input:  questions = [[3, 2], [4, 3], [4, 4], [2, 5]]
//! Output: 5
//! Explanation: solve question 0 (3 points, skip 1 and 2), then solve question 3 (2 points).
//!
//! Input:  questions = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]
//! Output: 7
//! Explanation: solve question 1 (2 points, skip 2 and 3), then question 4 (5 points).
//! ```
//!
//! ## Constraints
//!
//! - `1 <= questions.len() <= 10^5`
//! - `questions[i].len() == 2`
//! - `1 <= points, brainpower <= 10^5`
//!
//! ## Hints
//!
//! 1. Going forward, a decision affects which questions come *later*. Think about the problem from
//!    the end: what is the best score starting at question `i`?
//! 2. `best(i) = max(best(i + 1), points[i] + best(i + brainpower[i] + 1))`, where `best(k) = 0`
//!    for `k >= n`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** fill `best` from right to left. `best[n] = 0`; for each `i` from `n - 1` down to `0`,
//! either skip (`best[i + 1]`) or solve (`points + best[next]`, where `next = i + brainpower + 1`
//! may be past the end, in which case it contributes `0`). Return `best[0]`.
//!
//! **Complexity:** `O(n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `(0..n).rev()` walks indices backwards.
//! - `best.get(next).copied().unwrap_or(0)` reads an index that may be out of range without a
//!   manual bounds check (no panic).
//! - Converting with `i64::from(points)` is lossless, unlike `as`, and documents the widening.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        // @solution-start
        let n = questions.len();
        let mut best = vec![0_i64; n + 1];
        for i in (0..n).rev() {
            let points = i64::from(questions[i][0]);
            let next = i + questions[i][1] as usize + 1;
            let solve = points + best.get(next).copied().unwrap_or(0);
            best[i] = best[i + 1].max(solve);
        }
        best[0]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_questions() {
        let q = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        assert_eq!(Solution::most_points(q), 5);
    }

    #[test]
    fn example_five_questions() {
        let q = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        assert_eq!(Solution::most_points(q), 7);
    }

    #[test]
    fn single_question() {
        assert_eq!(Solution::most_points(vec![vec![7, 100]]), 7);
    }

    #[test]
    fn mixed_questions() {
        let q = vec![
            vec![21, 5],
            vec![92, 3],
            vec![74, 2],
            vec![39, 4],
            vec![58, 2],
            vec![5, 5],
            vec![49, 4],
            vec![65, 3],
        ];
        assert_eq!(Solution::most_points(q), 157);
    }

    #[test]
    fn result_needs_i64() {
        // Each question has brainpower 1, so every other question can be solved.
        let q = vec![vec![100_000, 1]; 100_000];
        assert_eq!(Solution::most_points(q), 100_000 * 50_000);
    }
}
