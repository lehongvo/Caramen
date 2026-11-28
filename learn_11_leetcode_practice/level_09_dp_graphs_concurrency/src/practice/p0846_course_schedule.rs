// ==================================================================
// PRACTICE 0846: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0846_course_schedule.md
// Run the tests:          cargo test practice::p0846
// ==================================================================

//! # Problem 0846: Course Schedule
//!
//! - **Difficulty:** Medium
//! - **Topics:** cycle detection in a directed graph, Kahn's algorithm (topological sort by
//!   in-degree), `VecDeque`
//! - **Source:** [LeetCode 207. Course Schedule](https://leetcode.com/problems/course-schedule/)
//!
//! ## Problem
//!
//! There are `num_courses` courses labelled `0..num_courses`. `prerequisites[i] = [a, b]` means
//! you must finish course `b` before you can take course `a`. Return `true` if it is possible to
//! finish all the courses, and `false` otherwise.
//!
//! Implement `Solution::can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num_courses = 2, prerequisites = [[1, 0]]
//! Output: true
//! Explanation: take course 0, then course 1.
//!
//! Input:  num_courses = 2, prerequisites = [[1, 0], [0, 1]]
//! Output: false
//! Explanation: each course requires the other one first.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num_courses <= 2000`
//! - `0 <= prerequisites.len() <= 5000`, `prerequisites[i].len() == 2`
//! - `0 <= a, b < num_courses`; all pairs are distinct.
//!
//! ## Hints
//!
//! 1. Draw an edge `b -> a` for every pair. All courses can be finished exactly when this directed
//!    graph has no cycle.
//! 2. A course with no remaining prerequisites (in-degree `0`) can be taken now. Taking it lowers
//!    the in-degree of the courses that depend on it.
//! 3. If you can take every course this way, there is no cycle.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, pre: &[[i32; 2]]) -> bool {
        Solution::can_finish(n, pre.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_possible() {
        assert!(run(2, &[[1, 0]]));
    }

    #[test]
    fn example_two_cycle() {
        assert!(!run(2, &[[1, 0], [0, 1]]));
    }

    #[test]
    fn no_prerequisites() {
        assert!(run(1, &[]));
        assert!(run(5, &[]));
    }

    #[test]
    fn cycle_hidden_in_a_bigger_graph() {
        // 0 -> 1 -> 2 is fine, but 3 -> 4 -> 5 -> 3 is a cycle.
        assert!(!run(6, &[[1, 0], [2, 1], [4, 3], [5, 4], [3, 5]]));
        assert!(run(6, &[[1, 0], [2, 1], [4, 3], [5, 4], [5, 0]]));
    }

    #[test]
    fn long_chain() {
        let pre: Vec<[i32; 2]> = (1..2000).map(|c| [c, c - 1]).collect();
        assert!(run(2000, &pre));
        let mut with_cycle = pre.clone();
        with_cycle.push([0, 1999]);
        assert!(!run(2000, &with_cycle));
    }
}
