//! # Problem 0360: Number of Recent Calls
//!
//! - **Difficulty:** Easy
//! - **Topics:** `VecDeque` sliding time window, `front`, `pop_front`, `while let ... if` loop, design struct
//! - **Source:** [LeetCode 933. Number of Recent Calls](https://leetcode.com/problems/number-of-recent-calls/)
//!
//! ## Problem
//!
//! A web service wants to know how many requests it received recently. Implement the
//! `RecentCounter` struct:
//!
//! - `RecentCounter::new() -> Self` starts with no requests;
//! - `ping(&mut self, t: i32) -> i32` records a request at time `t` (in milliseconds) and returns
//!   how many requests happened in the window `[t - 3000, t]` (both ends included), counting the new
//!   one.
//!
//! Every call uses a strictly larger `t` than the previous call.
//!
//! LeetCode's template declares `ping(&self, ...)`; as its comment allows, this version takes
//! `&mut self` because it changes the counter.
//!
//! ## Examples
//!
//! ```text
//! let mut c = RecentCounter::new();
//! c.ping(1)    => 1   (window [-2999, 1]: 1)
//! c.ping(100)  => 2   (window [-2900, 100]: 1, 100)
//! c.ping(3001) => 3   (window [1, 3001]: 1, 100, 3001)
//! c.ping(3002) => 3   (window [2, 3002]: 100, 3001, 3002)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= t <= 10^9`, strictly increasing across calls
//! - At most `10^4` calls to `ping`.
//!
//! ## Hints
//!
//! 1. Old requests never come back into the window, so they can be thrown away.
//! 2. Keep the times in a `VecDeque`: new times go to the back, expired ones leave from the front.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the times in the queue are increasing. On each ping, push `t` at the back, then pop
//! from the front while the oldest time is `< t - 3000`. What remains is exactly the window, and
//! its length is the answer.
//!
//! **Complexity:** `O(1)` amortized per call (each time is pushed and popped at most once),
//! `O(w)` space for the `w` requests inside a window (at most 3001).
//!
//! **Rust concepts:**
//! - `front()` peeks without removing: `while let Some(&oldest) = self.times.front()` reads the
//!   oldest time and the loop body decides whether to `pop_front()`.
//! - `VecDeque` is the natural "sliding window" container: push at one end, pop at the other.
//! - `#[allow(clippy::new_without_default)]` keeps LeetCode's `new()`-only template.
// @solution-end

use std::collections::VecDeque;

/// Counts the requests of the last 3000 milliseconds.
pub struct RecentCounter {
    // @solution-start remove
    times: VecDeque<i32>,
    // @solution-end
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl RecentCounter {
    pub fn new() -> Self {
        // @solution-start
        Self {
            times: VecDeque::new(),
        }
        // @solution-end
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        // @solution-start
        self.times.push_back(t);
        while let Some(&oldest) = self.times.front() {
            if oldest >= t - 3000 {
                break;
            }
            self.times.pop_front();
        }
        self.times.len() as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1), 1);
        assert_eq!(c.ping(100), 2);
        assert_eq!(c.ping(3001), 3);
        assert_eq!(c.ping(3002), 3);
    }

    #[test]
    fn window_bounds_are_inclusive() {
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1000), 1);
        assert_eq!(c.ping(4000), 2);
        assert_eq!(c.ping(4001), 2);
    }

    #[test]
    fn far_apart_pings() {
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1), 1);
        assert_eq!(c.ping(1_000_000), 1);
        assert_eq!(c.ping(1_000_000_000), 1);
    }

    #[test]
    fn dense_pings() {
        let mut c = RecentCounter::new();
        let mut last = 0;
        for t in 1..=10_000 {
            last = c.ping(t);
        }
        assert_eq!(last, 3001);
    }
}
