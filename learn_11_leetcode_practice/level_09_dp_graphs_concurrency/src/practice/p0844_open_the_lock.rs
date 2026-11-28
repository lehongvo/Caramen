// ==================================================================
// PRACTICE 0844: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0844_open_the_lock.md
// Run the tests:          cargo test practice::p0844
// ==================================================================

//! # Problem 0844: Open the Lock
//!
//! - **Difficulty:** Medium
//! - **Topics:** BFS on an implicit graph of states, encoding a state as a number, `[bool; 10_000]`
//! - **Source:** [LeetCode 752. Open the Lock](https://leetcode.com/problems/open-the-lock/)
//!
//! ## Problem
//!
//! A lock has four wheels, each showing a digit `0`-`9`. One **move** turns one wheel by one step
//! up or down; wheels wrap around (`9` goes up to `0` and `0` goes down to `9`). The lock starts at
//! `"0000"`. Some combinations are `deadends`: if the lock ever shows one of them, it jams, so you
//! can never pass through them. Return the minimum number of moves to reach `target`, or `-1` if it
//! is impossible.
//!
//! Implement `Solution::open_lock(deadends: Vec<String>, target: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  deadends = ["0201", "0101", "0102", "1212", "2002"], target = "0202"
//! Output: 6
//! Explanation: 0000 -> 1000 -> 1100 -> 1200 -> 1201 -> 1202 -> 0202.
//!
//! Input:  deadends = ["8888"], target = "0009"
//! Output: 1
//! Explanation: turn the last wheel down once.
//!
//! Input:  deadends = ["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
//!         target = "8888"
//! Output: -1
//! Explanation: every neighbour of the target is a dead end.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= deadends.len() <= 500`
//! - Every combination has exactly 4 digits; `target` is not in `deadends`.
//!
//! ## Hints
//!
//! 1. The 10 000 combinations are the nodes of a graph; each has 8 neighbours. Shortest number of
//!    moves = BFS distance.
//! 2. Encode `"abcd"` as the number `abcd` (`0..10_000`) so `visited` can be a plain array.
//! 3. If `"0000"` itself is a dead end, the answer is `-1`.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(dead: &[&str], target: &str) -> i32 {
        Solution::open_lock(
            dead.iter().map(|d| d.to_string()).collect(),
            target.to_string(),
        )
    }

    #[test]
    fn example_detour() {
        assert_eq!(run(&["0201", "0101", "0102", "1212", "2002"], "0202"), 6);
    }

    #[test]
    fn example_wrap_around() {
        assert_eq!(run(&["8888"], "0009"), 1);
    }

    #[test]
    fn example_target_surrounded() {
        let dead = [
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ];
        assert_eq!(run(&dead, "8888"), -1);
    }

    #[test]
    fn start_is_a_dead_end() {
        assert_eq!(run(&["0000"], "8888"), -1);
    }

    #[test]
    fn target_is_the_start() {
        assert_eq!(run(&["1111"], "0000"), 0);
    }

    #[test]
    fn farthest_combination() {
        // Each wheel needs 5 turns to reach 5 from 0, in either direction.
        assert_eq!(run(&["9999"], "5555"), 20);
    }
}
