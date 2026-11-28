// ==================================================================
// PRACTICE 0540: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0540_reward_top_k_students.md
// Run the tests:          cargo test practice::p0540
// ==================================================================

//! # Problem 0540: Reward Top K Students
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashSet<&str>` borrowing from the input, tuple sort keys `(Reverse(points), id)`, `sort_unstable_by_key`
//! - **Source:** [LeetCode 2512. Reward Top K Students](https://leetcode.com/problems/reward-top-k-students/)
//!
//! ## Problem
//!
//! Teachers write one feedback report per student. Every word of a report that appears in
//! `positive_feedback` gives the student `+3` points, and every word that appears in
//! `negative_feedback` gives `-1` point (other words give nothing). Students start with `0` points.
//!
//! The `i`-th report, `report[i]`, belongs to the student with id `student_id[i]`. Return the ids of the
//! `k` best students: sorted by points **descending**, and by id **ascending** on ties.
//!
//! Implement `Solution::top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>,
//! report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! positive = ["smart", "brilliant", "studious"], negative = ["not"]
//!
//! report = ["this student is studious", "the student is smart"], ids = [1, 2], k = 2
//! Output: [1, 2]      both have 3 points, so the smaller id comes first
//!
//! report = ["this student is not studious", "the student is smart"], ids = [1, 2], k = 2
//! Output: [2, 1]      student 1 has 3 - 1 = 2 points, student 2 has 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= positive_feedback.len(), negative_feedback.len() <= 10^4`
//! - Every feedback word has `1..=100` lowercase letters; no word is both positive and negative.
//! - `n == report.len() == student_id.len()`, `1 <= n <= 10^4`
//! - Reports are lowercase words separated by single spaces, at most 100 characters.
//! - Student ids are distinct, `1 <= student_id[i] <= 10^9`, and `1 <= k <= n`.
//!
//! ## Hints
//!
//! 1. Put the feedback words in `HashSet<&str>`s that borrow from the input vectors: there is no need
//!    to clone any word.
//! 2. Sorting `(points, id)` pairs by the key `(Reverse(points), id)` gives "points descending, id
//!    ascending" because tuples compare field by field.

use std::cmp::Reverse;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_tie_on_points() {
        let result = Solution::top_students(
            strings(&["smart", "brilliant", "studious"]),
            strings(&["not"]),
            strings(&["this student is studious", "the student is smart"]),
            vec![1, 2],
            2,
        );
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_negative_word() {
        let result = Solution::top_students(
            strings(&["smart", "brilliant", "studious"]),
            strings(&["not"]),
            strings(&["this student is not studious", "the student is smart"]),
            vec![1, 2],
            2,
        );
        assert_eq!(result, vec![2, 1]);
    }

    #[test]
    fn negative_totals_and_partial_k() {
        let result = Solution::top_students(
            strings(&["good"]),
            strings(&["bad", "late"]),
            strings(&["bad late bad", "good good", "nothing here", "late"]),
            vec![40, 30, 20, 10],
            3,
        );
        // points: 40 -> -3, 30 -> 6, 20 -> 0, 10 -> -1
        assert_eq!(result, vec![30, 20, 10]);
    }

    #[test]
    fn many_students() {
        let n = 10_000;
        let reports: Vec<String> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    "great".to_string()
                } else {
                    "meh".to_string()
                }
            })
            .collect();
        let ids: Vec<i32> = (0..n).map(|i| 1_000_000_000 - i).collect();
        let result =
            Solution::top_students(strings(&["great"]), strings(&["meh"]), reports, ids, 3);
        // Even positions score 3; among them the smallest ids are the last even positions.
        assert_eq!(result, vec![999_990_002, 999_990_004, 999_990_006]);
    }
}
