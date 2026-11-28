// ==================================================================
// PRACTICE 0977: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0977_course_schedule_iii.md
// Run the tests:          cargo test practice::p0977
// ==================================================================

//! # Problem 0977: Course Schedule III
//!
//! - **Difficulty:** Hard
//! - **Topics:** exchange-argument greedy, sorting by deadline with `sort_unstable_by_key`, a max-heap of durations to undo choices
//! - **Source:** [LeetCode 630. Course Schedule III](https://leetcode.com/problems/course-schedule-iii/)
//!
//! ## Problem
//!
//! `courses[i] = [duration, last_day]`: course `i` takes `duration` consecutive days and must be
//! **finished** by day `last_day`. You start on day 1 and take one course at a time. Return the
//! maximum number of courses you can complete.
//!
//! Implement `Solution::schedule_course(courses: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]
//! Output: 3
//!
//! Input:  courses = [[1, 2]]           Output: 1
//! Input:  courses = [[3, 2], [4, 3]]   Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= courses.len() <= 10^4`
//! - `1 <= duration, last_day <= 10^4`
//!
//! ## Hints
//!
//! 1. Consider courses in order of their deadline: any feasible set can be taken in that order.
//! 2. Take each course; if the total time now exceeds its deadline, drop the **longest** course taken
//!    so far (possibly the new one). The count stays the same and the total time shrinks the most.
//! 3. A max-heap of the durations taken finds the longest one in `O(log n)`.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(courses: &[[i32; 2]]) -> i32 {
        Solution::schedule_course(courses.iter().map(|c| c.to_vec()).collect())
    }

    fn naive(courses: &[[i32; 2]]) -> i32 {
        let n = courses.len();
        let mut best = 0;
        for mask in 0..(1_u32 << n) {
            let mut chosen: Vec<[i32; 2]> = (0..n)
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| courses[i])
                .collect();
            chosen.sort_unstable_by_key(|c| c[1]);
            let mut time = 0;
            if chosen.iter().all(|c| {
                time += c[0];
                time <= c[1]
            }) {
                best = best.max(chosen.len() as i32);
            }
        }
        best
    }

    #[test]
    fn examples() {
        assert_eq!(
            run(&[[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]),
            3
        );
        assert_eq!(run(&[[1, 2]]), 1);
        assert_eq!(run(&[[3, 2], [4, 3]]), 0);
    }

    #[test]
    fn swapping_a_long_course_for_short_ones() {
        assert_eq!(run(&[[5, 5], [4, 6], [2, 6]]), 2);
        assert_eq!(
            run(&[
                [7, 17],
                [3, 12],
                [10, 20],
                [9, 10],
                [5, 20],
                [10, 19],
                [4, 18]
            ]),
            4
        );
    }

    #[test]
    fn matches_brute_force() {
        let courses: Vec<[i32; 2]> = (0..12)
            .map(|i: i32| [(i * 7) % 9 + 1, (i * 11) % 25 + 5])
            .collect();
        assert_eq!(run(&courses), naive(&courses));
    }

    #[test]
    fn many_short_courses() {
        let courses: Vec<[i32; 2]> = (0..10_000).map(|_| [1, 10_000]).collect();
        assert_eq!(run(&courses), 10_000);
        let tight: Vec<[i32; 2]> = (0..10_000).map(|_| [2, 10_000]).collect();
        assert_eq!(run(&tight), 5_000);
    }
}
