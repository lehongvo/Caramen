// ==================================================================
// PRACTICE 1000: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p1000_bus_routes.md
// Run the tests:          cargo test practice::p1000
// ==================================================================

//! # Problem 1000: Bus Routes
//!
//! - **Difficulty:** Hard
//! - **Topics:** BFS over **routes** instead of stops, `HashMap<stop, Vec<route>>` as an inverted index, visited sets for both routes and stops
//! - **Source:** [LeetCode 815. Bus Routes](https://leetcode.com/problems/bus-routes/)
//!
//! ## Problem
//!
//! `routes[i]` lists the stops of bus `i`, which drives them in a loop forever (so you can travel
//! between any two stops of the same route on that bus). Starting at stop `source` (not on any bus
//! yet), return the minimum number of **buses** you must take to reach stop `target`, or `-1` if it
//! is impossible. If `source == target`, the answer is `0`.
//!
//! Implement `Solution::num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! routes = [[1, 2, 7], [3, 6, 7]], source = 1, target = 6                 => 2
//! routes = [[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]],
//!          source = 15, target = 12                                         => -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= routes.len() <= 500`, `1 <= routes[i].len() <= 10^5`, total stops `<= 10^5`
//! - The stops of one route are distinct; `0 <= stop, source, target < 10^6`.
//!
//! ## Hints
//!
//! 1. Build `stop -> list of routes` once.
//! 2. BFS where one step is "take one more bus": the first layer is every route through `source`.
//! 3. Expanding a route visits all its stops; mark routes **and** stops as visited so every route and
//!    stop is processed at most once (otherwise long routes are scanned again and again).

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(routes: &[&[i32]], source: i32, target: i32) -> i32 {
        Solution::num_buses_to_destination(
            routes.iter().map(|r| r.to_vec()).collect(),
            source,
            target,
        )
    }

    #[test]
    fn examples() {
        assert_eq!(run(&[&[1, 2, 7], &[3, 6, 7]], 1, 6), 2);
        assert_eq!(
            run(
                &[&[7, 12], &[4, 5, 15], &[6], &[15, 19], &[9, 12, 13]],
                15,
                12
            ),
            -1
        );
    }

    #[test]
    fn trivial_cases() {
        assert_eq!(run(&[&[1, 2]], 5, 5), 0);
        assert_eq!(run(&[&[1, 2]], 1, 2), 1);
        assert_eq!(run(&[&[1, 2]], 3, 2), -1);
        assert_eq!(run(&[&[1, 2]], 1, 3), -1);
    }

    #[test]
    fn transfers_along_a_chain() {
        let routes: Vec<Vec<i32>> = (0..500).map(|i| vec![i, i + 1]).collect();
        let refs: Vec<&[i32]> = routes.iter().map(Vec::as_slice).collect();
        assert_eq!(run(&refs, 0, 500), 500);
        assert_eq!(run(&refs, 250, 0), 250);
        assert_eq!(run(&refs, 499, 500), 1);
    }

    #[test]
    fn one_huge_route() {
        let big: Vec<i32> = (0..100_000).collect();
        assert_eq!(run(&[&big, &[99_999, 500_000]], 0, 500_000), 2);
    }
}
