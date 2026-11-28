// ==================================================================
// PRACTICE 0847: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0847_course_schedule_ii.md
// Run the tests:          cargo test practice::p0847
// ==================================================================

//! # Problem 0847: Course Schedule II
//!
//! - **Difficulty:** Medium
//! - **Topics:** topological order with Kahn's algorithm, returning the order itself, testing an
//!   answer that is not unique
//! - **Source:** [LeetCode 210. Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)
//!
//! ## Problem
//!
//! Same setting as problem 0846: `prerequisites[i] = [a, b]` means course `b` must be finished
//! before course `a`. This time return an **order** in which all `num_courses` courses can be
//! taken. If several orders exist, return any of them. If it is impossible (there is a cycle),
//! return an empty vector.
//!
//! Implement `Solution::find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num_courses = 2, prerequisites = [[1, 0]]
//! Output: [0, 1]
//!
//! Input:  num_courses = 4, prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]]
//! Output: [0, 1, 2, 3]   (or [0, 2, 1, 3])
//!
//! Input:  num_courses = 1, prerequisites = []
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num_courses <= 2000`
//! - `0 <= prerequisites.len() <= num_courses * (num_courses - 1)`
//! - `0 <= a, b < num_courses`, `a != b`; all pairs are distinct.
//!
//! ## Hints
//!
//! 1. Kahn's algorithm (problem 0846) takes the courses in a valid order: record them.
//! 2. If fewer than `num_courses` courses were taken, a cycle blocked the rest.

pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that `order` contains every course once and respects every prerequisite.
    fn assert_valid(n: i32, pre: &[[i32; 2]], order: &[i32]) {
        assert_eq!(order.len(), n as usize, "every course must appear");
        let mut pos = vec![usize::MAX; n as usize];
        for (i, &c) in order.iter().enumerate() {
            assert_eq!(pos[c as usize], usize::MAX, "course {c} appears twice");
            pos[c as usize] = i;
        }
        for &[a, b] in pre {
            assert!(
                pos[b as usize] < pos[a as usize],
                "{b} must come before {a}"
            );
        }
    }

    fn run(n: i32, pre: &[[i32; 2]]) -> Vec<i32> {
        Solution::find_order(n, pre.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_two_courses() {
        assert_eq!(run(2, &[[1, 0]]), vec![0, 1]);
    }

    #[test]
    fn example_diamond() {
        let pre = [[1, 0], [2, 0], [3, 1], [3, 2]];
        assert_valid(4, &pre, &run(4, &pre));
    }

    #[test]
    fn example_single_course() {
        assert_eq!(run(1, &[]), vec![0]);
    }

    #[test]
    fn cycle_gives_empty_order() {
        assert!(run(3, &[[1, 0], [2, 1], [0, 2]]).is_empty());
    }

    #[test]
    fn larger_dag() {
        // Course c requires every course that divides it (c > d >= 1, d | c); course 0 is free.
        let mut pre = Vec::new();
        for c in 2..300 {
            for d in 1..c {
                if c % d == 0 {
                    pre.push([c, d]);
                }
            }
        }
        assert_valid(300, &pre, &run(300, &pre));
    }
}
