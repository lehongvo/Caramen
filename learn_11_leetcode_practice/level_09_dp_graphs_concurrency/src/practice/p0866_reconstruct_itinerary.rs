// ==================================================================
// PRACTICE 0866: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0866_reconstruct_itinerary.md
// Run the tests:          cargo test practice::p0866
// ==================================================================

//! # Problem 0866: Reconstruct Itinerary
//!
//! - **Difficulty:** Hard
//! - **Topics:** Eulerian path with Hierholzer's algorithm, `HashMap<&str, Vec<&str>>` sorted in
//!   reverse so `pop` yields the smallest, iterative stack
//! - **Source:** [LeetCode 332. Reconstruct Itinerary](https://leetcode.com/problems/reconstruct-itinerary/)
//!
//! ## Problem
//!
//! `tickets[i] = [from, to]` is a one-way flight between airports (three capital letters each).
//! Using **every ticket exactly once**, build an itinerary that starts at `"JFK"`. At least one
//! valid itinerary is guaranteed to exist. If several exist, return the one that is smallest in
//! lexicographic order when read as a list of airports (for example `["JFK", "LGA"]` comes before
//! `["JFK", "LGB"]`).
//!
//! Implement `Solution::find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  tickets = [["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]]
//! Output: ["JFK", "MUC", "LHR", "SFO", "SJC"]
//!
//! Input:  tickets = [["JFK", "SFO"], ["JFK", "ATL"], ["SFO", "ATL"], ["ATL", "JFK"],
//!                    ["ATL", "SFO"]]
//! Output: ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
//! Explanation: ["JFK", "SFO", "ATL", "JFK", "ATL", "SFO"] is also valid but larger.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= tickets.len() <= 300`
//! - Airport codes are 3 uppercase letters; `from != to`; tickets may repeat.
//!
//! ## Hints
//!
//! 1. Always taking the smallest destination first does not work alone: it can walk into a dead end
//!    while tickets remain (try `[["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]]`).
//! 2. Hierholzer's algorithm: walk greedily (smallest first) until you are stuck; the airport where
//!    you get stuck is the **end** of the remaining itinerary. Back up and continue from the
//!    previous airport.
//! 3. Airports are appended to the route when you get stuck, so the route comes out reversed.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(tickets: &[[&str; 2]]) -> Vec<String> {
        let t = tickets
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect())
            .collect();
        Solution::find_itinerary(t)
    }

    #[test]
    fn example_straight_line() {
        let t = [
            ["MUC", "LHR"],
            ["JFK", "MUC"],
            ["SFO", "SJC"],
            ["LHR", "SFO"],
        ];
        assert_eq!(run(&t), vec!["JFK", "MUC", "LHR", "SFO", "SJC"]);
    }

    #[test]
    fn example_smallest_of_several() {
        let t = [
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"],
        ];
        assert_eq!(run(&t), vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);
    }

    #[test]
    fn greedy_dead_end() {
        // Taking "KUL" first would strand the NRT tickets.
        let t = [["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]];
        assert_eq!(run(&t), vec!["JFK", "NRT", "JFK", "KUL"]);
    }

    #[test]
    fn repeated_tickets() {
        let t = [
            ["JFK", "AAA"],
            ["AAA", "JFK"],
            ["JFK", "AAA"],
            ["AAA", "JFK"],
        ];
        assert_eq!(run(&t), vec!["JFK", "AAA", "JFK", "AAA", "JFK"]);
    }

    #[test]
    fn many_round_trips() {
        // 150 round trips JFK -> XYZ -> JFK: every ticket is used.
        let mut t = Vec::new();
        for _ in 0..150 {
            t.push(["JFK", "XYZ"]);
            t.push(["XYZ", "JFK"]);
        }
        let got = run(&t);
        assert_eq!(got.len(), 301);
        assert!(got.iter().step_by(2).all(|a| a == "JFK"));
    }
}
