// ==================================================================
// PRACTICE 0357: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0357_events_in_a_time_window.md
// Run the tests:          cargo test practice::p0357
// ==================================================================

//! # Problem 0357: Events in a Time Window
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BTreeMap::range` with `a..b`, `..=b`, `a..` and `(Bound::Excluded(a), Bound::Unbounded)`, `flat_map`
//! - **Source:** Original · [`BTreeMap::range`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range) · [`std::ops::Bound`](https://doc.rust-lang.org/std/ops/enum.Bound.html)
//!
//! ## Problem
//!
//! A `Timeline` stores named events at integer timestamps (several events may share a timestamp).
//! Implement it on top of a `BTreeMap<u32, Vec<String>>` so that time-window queries do not scan
//! every event:
//!
//! - `Timeline::new() -> Self` (also `Timeline::default()`): an empty timeline;
//! - `add(&mut self, time: u32, name: &str)`: record an event;
//! - `between(&self, start: u32, end: u32) -> Vec<String>`: events with `start <= time < end`;
//!   an empty vector when `start >= end`;
//! - `up_to(&self, time: u32) -> Vec<String>`: events with `t <= time`;
//! - `after(&self, time: u32) -> Vec<String>`: events with `t > time` (strictly later);
//! - `next_time(&self, time: u32) -> Option<u32>`: the earliest timestamp `>= time` that has an
//!   event.
//!
//! Every returned list is in time order; events with the same timestamp appear in the order they
//! were added.
//!
//! ## Examples
//!
//! ```text
//! add(10, "boot"); add(20, "login"); add(20, "mail"); add(35, "logout")
//! between(10, 20) => ["boot"]                 (20 is excluded)
//! between(15, 40) => ["login", "mail", "logout"]
//! between(30, 30) => []
//! up_to(20)       => ["boot", "login", "mail"]
//! after(20)       => ["logout"]
//! next_time(21)   => Some(35)
//! next_time(36)   => None
//! ```
//!
//! ## Constraints
//!
//! - At most `10^5` events; any `u32` timestamp.
//!
//! ## Hints
//!
//! 1. `map.range(a..b)` iterates over the entries with keys in `a..b`, in key order, in
//!    `O(log n + k)`.
//! 2. `range` **panics** if the start is greater than the end, so check `start >= end` first.
//! 3. For "strictly greater than", pass a pair of bounds:
//!    `(Bound::Excluded(time), Bound::Unbounded)`.

use std::collections::BTreeMap;
use std::ops::Bound;

/// Named events stored by timestamp.
#[derive(Debug, Default)]
pub struct Timeline {}

impl Timeline {
    /// Creates an empty timeline.
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Records the event `name` at `time`.
    pub fn add(&mut self, time: u32, name: &str) {
        todo!("Write your solution here")
    }

    /// Events with `start <= t < end`.
    pub fn between(&self, start: u32, end: u32) -> Vec<String> {
        todo!("Write your solution here")
    }

    /// Events with `t <= time`.
    pub fn up_to(&self, time: u32) -> Vec<String> {
        todo!("Write your solution here")
    }

    /// Events with `t > time`.
    pub fn after(&self, time: u32) -> Vec<String> {
        todo!("Write your solution here")
    }

    /// The earliest timestamp `>= time` that has an event.
    pub fn next_time(&self, time: u32) -> Option<u32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Timeline {
        let mut t = Timeline::new();
        t.add(20, "login");
        t.add(10, "boot");
        t.add(35, "logout");
        t.add(20, "mail");
        t
    }

    #[test]
    fn between_examples() {
        let t = example();
        assert_eq!(t.between(10, 20), vec!["boot"]);
        assert_eq!(t.between(15, 40), vec!["login", "mail", "logout"]);
        assert_eq!(t.between(30, 30), Vec::<String>::new());
        assert_eq!(t.between(40, 30), Vec::<String>::new());
    }

    #[test]
    fn up_to_and_after() {
        let t = example();
        assert_eq!(t.up_to(20), vec!["boot", "login", "mail"]);
        assert_eq!(t.up_to(9), Vec::<String>::new());
        assert_eq!(t.after(20), vec!["logout"]);
        assert_eq!(t.after(u32::MAX), Vec::<String>::new());
        assert_eq!(t.after(0), vec!["boot", "login", "mail", "logout"]);
    }

    #[test]
    fn next_time_examples() {
        let t = example();
        assert_eq!(t.next_time(21), Some(35));
        assert_eq!(t.next_time(20), Some(20));
        assert_eq!(t.next_time(36), None);
        assert_eq!(Timeline::default().next_time(0), None);
    }

    #[test]
    fn many_events() {
        let mut t = Timeline::new();
        for i in (0..100_000u32).rev() {
            t.add(i * 10, &format!("e{i}"));
        }
        assert_eq!(t.between(500, 550), vec!["e50", "e51", "e52", "e53", "e54"]);
        assert_eq!(t.up_to(25).len(), 3);
        assert_eq!(t.after(999_980), vec!["e99999"]);
        assert_eq!(t.next_time(999_991), None);
        assert_eq!(t.next_time(11), Some(20));
    }
}
