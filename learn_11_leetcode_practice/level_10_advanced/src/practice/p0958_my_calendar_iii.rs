// ==================================================================
// PRACTICE 0958: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0958_my_calendar_iii.md
// Run the tests:          cargo test practice::p0958
// ==================================================================

//! # Problem 0958: My Calendar III
//!
//! - **Difficulty:** Hard
//! - **Topics:** sweep line over a `BTreeMap` of boundary deltas, `entry().or_insert(0)`, `scan` for running sums
//! - **Source:** [LeetCode 732. My Calendar III](https://leetcode.com/problems/my-calendar-iii/)
//!
//! ## Problem
//!
//! Events are half-open time intervals `[start_time, end_time)`. A **k-booking** happens when `k`
//! events share some moment in time. Every booking is accepted (there is no limit).
//!
//! Implement the `MyCalendarThree` struct:
//!
//! - `MyCalendarThree::new() -> Self`.
//! - `book(&mut self, start_time: i32, end_time: i32) -> i32`: adds the event and returns the largest
//!   `k` such that some moment is covered by `k` of the events booked so far.
//!
//! ## Examples
//!
//! ```text
//! book(10, 20) => 1
//! book(50, 60) => 1
//! book(10, 40) => 2
//! book(5, 15)  => 3
//! book(5, 10)  => 3
//! book(25, 55) => 3
//! ```
//!
//! ## Constraints
//!
//! - `0 <= start_time < end_time <= 10^9`
//! - At most `400` calls to `book`.
//!
//! ## Hints
//!
//! 1. Record `+1` at every start and `-1` at every end in a sorted map of time to delta.
//! 2. Walking the map in time order, the running sum of deltas is the number of events active just
//!    after each boundary; its maximum is the answer.
//! 3. Half-open intervals make `[5, 10)` and `[10, 20)` not overlap: at time `10`, the `-1` and the
//!    `+1` are summed in the same entry before the maximum is taken.

use std::collections::BTreeMap;

pub struct MyCalendarThree {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyCalendarThree {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut c = MyCalendarThree::new();
        let results: Vec<i32> = [(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)]
            .iter()
            .map(|&(s, e)| c.book(s, e))
            .collect();
        assert_eq!(results, vec![1, 1, 2, 3, 3, 3]);
    }

    #[test]
    fn touching_events_do_not_overlap() {
        let mut c = MyCalendarThree::new();
        assert_eq!(c.book(0, 10), 1);
        assert_eq!(c.book(10, 20), 1);
        assert_eq!(c.book(20, 30), 1);
        assert_eq!(c.book(5, 25), 2);
    }

    #[test]
    fn identical_events_stack() {
        let mut c = MyCalendarThree::new();
        for k in 1..=400 {
            assert_eq!(c.book(0, 1_000_000_000), k);
        }
    }

    #[test]
    fn matches_brute_force() {
        let mut c = MyCalendarThree::new();
        let mut events: Vec<(i32, i32)> = Vec::new();
        for i in 0..200_i32 {
            let start = (i * 37) % 500;
            let end = start + (i * 13) % 60 + 1;
            events.push((start, end));
            let expected = (0..600)
                .map(|t| events.iter().filter(|&&(s, e)| s <= t && t < e).count() as i32)
                .max()
                .unwrap_or(0);
            assert_eq!(c.book(start, end), expected, "after {} events", i + 1);
        }
    }
}
