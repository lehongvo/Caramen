// ==================================================================
// PRACTICE 0538: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0538_ordering_for_tasks.md
// Run the tests:          cargo test practice::p0538
// ==================================================================

//! # Problem 0538: Ordering Tasks by Priority
//!
//! - **Difficulty:** Medium
//! - **Topics:** manual `Ord` and `PartialOrd`, `Ordering::then_with`, consistency with `Eq`, `BinaryHeap` as a max-heap, `std::cmp::Reverse`
//! - **Source:** Original · [std::cmp::Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord)
//!
//! ## Problem
//!
//! A task has a `priority: u8`, a `deadline: u32` and a `name: String` (all `pub`). Define the order
//! "`a > b` means `a` should run **before** `b`":
//!
//! 1. Higher `priority` runs first.
//! 2. On equal priority, the **earlier** `deadline` runs first.
//! 3. On equal priority and deadline, the name that comes first **alphabetically** runs first.
//!
//! Implement `Ord` (with this order) and `PartialOrd` (delegating to `Ord`) for `Task`. `PartialEq` and
//! `Eq` are derived; your `Ord` must agree with them (two tasks compare `Equal` exactly when all fields
//! are equal).
//!
//! Then:
//!
//! - `next_task(tasks: &[Task]) -> Option<&Task>`: the task that runs first (the maximum).
//! - `schedule(tasks: Vec<Task>) -> Vec<String>`: all task names in execution order, produced by
//!   pushing every task into a `std::collections::BinaryHeap` and popping until it is empty.
//! - `latest_first(tasks: Vec<Task>) -> Vec<String>`: the names sorted by **deadline descending**
//!   (ties: the order of `schedule`), using `sort_by_key` with `std::cmp::Reverse`.
//!
//! ## Examples
//!
//! ```text
//! tasks = [("email", p1, d30), ("deploy", p3, d50), ("review", p3, d20), ("backup", p3, d20)]
//! schedule(tasks)     => ["backup", "review", "deploy", "email"]
//! latest_first(tasks) => ["deploy", "email", "backup", "review"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= tasks.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `self.priority.cmp(&other.priority)` puts the higher priority on the "greater" side.
//! 2. For "earlier deadline is greater", compare the other way round: `other.deadline.cmp(&self.deadline)`.
//! 3. `BinaryHeap::pop` returns the **greatest** element, which is exactly the next task to run.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub priority: u8,
    pub deadline: u32,
    pub name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

/// The task that runs first.
pub fn next_task(tasks: &[Task]) -> Option<&Task> {
    todo!("Write your solution here")
}

/// Task names in execution order, via a `BinaryHeap`.
pub fn schedule(tasks: Vec<Task>) -> Vec<String> {
    todo!("Write your solution here")
}

/// Task names by deadline descending; ties in execution order.
pub fn latest_first(tasks: Vec<Task>) -> Vec<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(name: &str, priority: u8, deadline: u32) -> Task {
        Task {
            priority,
            deadline,
            name: name.to_string(),
        }
    }

    fn sample() -> Vec<Task> {
        vec![
            task("email", 1, 30),
            task("deploy", 3, 50),
            task("review", 3, 20),
            task("backup", 3, 20),
        ]
    }

    #[test]
    fn comparisons_follow_the_rules() {
        assert!(task("a", 2, 99) > task("b", 1, 1));
        assert!(task("a", 2, 10) > task("b", 2, 11));
        assert!(task("a", 2, 10) > task("b", 2, 10));
        assert_eq!(task("a", 2, 10).cmp(&task("a", 2, 10)), Ordering::Equal);
        assert_eq!(
            task("x", 0, 0).partial_cmp(&task("y", 0, 0)),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn example_schedule() {
        assert_eq!(
            schedule(sample()),
            vec!["backup", "review", "deploy", "email"]
        );
        assert_eq!(
            next_task(&sample()).map(|t| t.name.as_str()),
            Some("backup")
        );
    }

    #[test]
    fn example_latest_first() {
        assert_eq!(
            latest_first(sample()),
            vec!["deploy", "email", "backup", "review"]
        );
    }

    #[test]
    fn empty_inputs() {
        assert!(schedule(vec![]).is_empty());
        assert!(next_task(&[]).is_none());
        assert!(latest_first(vec![]).is_empty());
    }

    #[test]
    fn heap_matches_sorting() {
        let tasks: Vec<Task> = (0..100_000_u32)
            .map(|i| task(&format!("t{:05}", i % 977), (i % 7) as u8, i % 13))
            .collect();
        let mut sorted = tasks.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        let expected: Vec<String> = sorted.into_iter().map(|t| t.name).collect();
        assert_eq!(schedule(tasks), expected);
    }
}
