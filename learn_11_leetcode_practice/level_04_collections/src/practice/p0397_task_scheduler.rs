// ==================================================================
// PRACTICE 0397: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0397_task_scheduler.md
// Run the tests:          cargo test practice::p0397
// ==================================================================

//! # Problem 0397: Task Scheduler
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting into `[usize; 26]`, a max-heap of remaining counts, a cooldown `VecDeque<(ready_time, count)>`
//! - **Source:** [LeetCode 621. Task Scheduler](https://leetcode.com/problems/task-scheduler/)
//!
//! ## Problem
//!
//! A CPU must run a list of tasks, each labelled with an uppercase letter; tasks with the same
//! label are identical. In every time unit (interval) the CPU either runs **one** task or stays
//! **idle**. Two runs of the **same** label must be separated by at least `n` intervals (so after
//! running `A` at time `t`, the next `A` can run at time `t + n + 1` at the earliest). Tasks may run
//! in any order.
//!
//! Return the minimum number of intervals needed to run all tasks.
//!
//! Implement `Solution::least_interval(tasks: Vec<char>, n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  tasks = ['A', 'A', 'A', 'B', 'B', 'B'], n = 2
//! Output: 8       (A B idle A B idle A B)
//!
//! Input:  tasks = ['A', 'C', 'A', 'B', 'D', 'B'], n = 1
//! Output: 6       (A B C A B D: no idle time needed)
//!
//! Input:  tasks = ['A', 'A', 'A', 'B', 'B', 'B'], n = 3
//! Output: 10      (A B idle idle A B idle idle A B)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= tasks.len() <= 10^4`, uppercase English letters
//! - `0 <= n <= 100`
//!
//! ## Hints
//!
//! 1. Greedy: at every interval, run the available task with the **most** remaining copies.
//! 2. A max-heap holds the remaining counts of available labels. A label that just ran waits in a
//!    queue with the time at which it becomes available again.

use std::collections::{BinaryHeap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tasks(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(Solution::least_interval(tasks("AAABBB"), 2), 8);
    }

    #[test]
    fn example_two_no_idle() {
        assert_eq!(Solution::least_interval(tasks("ACABDB"), 1), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::least_interval(tasks("AAABBB"), 3), 10);
    }

    #[test]
    fn no_cooldown_and_single_label() {
        assert_eq!(Solution::least_interval(tasks("AAABBB"), 0), 6);
        assert_eq!(Solution::least_interval(tasks("AAAA"), 2), 10);
        assert_eq!(Solution::least_interval(tasks("A"), 100), 1);
    }

    #[test]
    fn many_tasks_match_the_formula() {
        let all: Vec<char> = (0..10_000)
            .map(|i| (b'A' + (i % 7 * i % 26) as u8) as char)
            .collect();
        for n in [0, 1, 5, 50, 100] {
            let mut counts = [0usize; 26];
            for &t in &all {
                counts[(t as u8 - b'A') as usize] += 1;
            }
            let max = *counts.iter().max().unwrap();
            let ties = counts.iter().filter(|&&c| c == max).count();
            let formula = all.len().max((max - 1) * (n + 1) + ties) as i32;
            assert_eq!(Solution::least_interval(all.clone(), n as i32), formula);
        }
    }
}
