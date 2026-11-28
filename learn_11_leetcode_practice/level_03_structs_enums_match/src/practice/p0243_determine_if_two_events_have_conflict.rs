// ==================================================================
// PRACTICE 0243: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0243_determine_if_two_events_have_conflict.md
// Run the tests:          cargo test practice::p0243
// ==================================================================

//! # Problem 0243: Determine if Two Events Have Conflict
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Time` struct with derived `PartialOrd`/`Ord`, an `Event` struct of two times, parsing with `split_once`
//! - **Source:** [LeetCode 2446. Determine if Two Events Have Conflict](https://leetcode.com/problems/determine-if-two-events-have-conflict/)
//!
//! ## Problem
//!
//! Two events happen on the same day. Each one is given as a list of two strings
//! `[start, end]` in 24-hour `"HH:MM"` format, and both ends are **inclusive**. The events conflict
//! if there is at least one moment that belongs to both of them (touching ends count: an event
//! ending at `"10:00"` conflicts with one starting at `"10:00"`).
//!
//! Return `true` if `event1` and `event2` conflict.
//!
//! Implement `Solution::have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool`.
//!
//! For this level, parse times into a `struct Time { hour, minute }` that **derives** `PartialOrd`,
//! so times can be compared with `<=` directly.
//!
//! ## Examples
//!
//! ```text
//! Input:  event1 = ["01:15", "02:00"], event2 = ["02:00", "03:00"]
//! Output: true     (they share 02:00)
//!
//! Input:  event1 = ["01:00", "02:00"], event2 = ["01:20", "03:00"]
//! Output: true
//!
//! Input:  event1 = ["10:00", "11:00"], event2 = ["14:00", "15:00"]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - Each event has exactly two valid times, and `start <= end`.
//!
//! ## Hints
//!
//! 1. With fields declared `hour` then `minute`, the derived order compares hours first, which is
//!    exactly chronological order.
//! 2. Two closed intervals `[a, b]` and `[c, d]` overlap exactly when `a <= d && c <= b`.

pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(start: &str, end: &str) -> Vec<String> {
        vec![start.to_string(), end.to_string()]
    }

    #[test]
    fn example_touching_ends() {
        assert!(Solution::have_conflict(
            ev("01:15", "02:00"),
            ev("02:00", "03:00")
        ));
    }

    #[test]
    fn example_overlap() {
        assert!(Solution::have_conflict(
            ev("01:00", "02:00"),
            ev("01:20", "03:00")
        ));
    }

    #[test]
    fn example_disjoint() {
        assert!(!Solution::have_conflict(
            ev("10:00", "11:00"),
            ev("14:00", "15:00")
        ));
    }

    #[test]
    fn one_inside_the_other_and_order_swapped() {
        assert!(Solution::have_conflict(
            ev("08:00", "18:00"),
            ev("12:30", "12:31")
        ));
        assert!(!Solution::have_conflict(
            ev("14:00", "15:00"),
            ev("10:00", "13:59")
        ));
    }

    #[test]
    fn minutes_decide_when_hours_match() {
        assert!(!Solution::have_conflict(
            ev("09:00", "09:29"),
            ev("09:30", "09:45")
        ));
        assert!(Solution::have_conflict(
            ev("00:00", "23:59"),
            ev("23:59", "23:59")
        ));
    }
}
