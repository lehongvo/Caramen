// ==================================================================
// PRACTICE 0857: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0857_network_delay_time.md
// Run the tests:          cargo test practice::p0857
// ==================================================================

//! # Problem 0857: Network Delay Time
//!
//! - **Difficulty:** Medium
//! - **Topics:** Dijkstra's algorithm, `BinaryHeap` as a min-heap with `std::cmp::Reverse`,
//!   lazy deletion of stale heap entries
//! - **Source:** [LeetCode 743. Network Delay Time](https://leetcode.com/problems/network-delay-time/)
//!
//! ## Problem
//!
//! A network has `n` nodes labelled `1..=n`. `times[i] = [u, v, w]` is a **directed** link: a
//! signal sent from `u` reaches `v` after `w` time units. A signal is sent from node `k`. Return the
//! time it takes until **all** `n` nodes have received it, or `-1` if some node never receives it.
//!
//! Implement `Solution::network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  times = [[2, 1, 1], [2, 3, 1], [3, 4, 1]], n = 4, k = 2
//! Output: 2
//! Explanation: node 4 is reached last, through 2 -> 3 -> 4.
//!
//! Input:  times = [[1, 2, 1]], n = 2, k = 1
//! Output: 1
//!
//! Input:  times = [[1, 2, 1]], n = 2, k = 2
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= n <= 100`
//! - `1 <= times.len() <= 6000`; `1 <= u, v <= n`, `u != v`; `0 <= w <= 100`
//! - All pairs `(u, v)` are distinct.
//!
//! ## Hints
//!
//! 1. Each node receives the signal at its shortest-path distance from `k`. The answer is the
//!    largest of those distances.
//! 2. All weights are non-negative, so Dijkstra's algorithm applies: always settle the unsettled
//!    node with the smallest known distance.
//! 3. `BinaryHeap` is a max-heap; wrap entries in `Reverse` to pop the smallest distance first.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(times: &[[i32; 3]], n: i32, k: i32) -> i32 {
        Solution::network_delay_time(times.iter().map(|t| t.to_vec()).collect(), n, k)
    }

    #[test]
    fn example_four_nodes() {
        assert_eq!(run(&[[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2), 2);
    }

    #[test]
    fn example_two_nodes() {
        assert_eq!(run(&[[1, 2, 1]], 2, 1), 1);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(run(&[[1, 2, 1]], 2, 2), -1);
    }

    #[test]
    fn detour_beats_direct_link() {
        assert_eq!(run(&[[1, 2, 10], [1, 3, 1], [3, 2, 1]], 3, 1), 2);
    }

    #[test]
    fn zero_weight_links() {
        assert_eq!(run(&[[1, 2, 0], [2, 3, 0]], 3, 1), 0);
        assert_eq!(run(&[[2, 1, 0], [1, 2, 7]], 2, 2), 0);
    }

    #[test]
    fn dense_graph() {
        // Complete digraph on 100 nodes with w(u, v) = |u - v|: the farthest node from 1 is 100.
        let mut times = Vec::new();
        for u in 1..=100_i32 {
            for v in 1..=100 {
                if u != v {
                    times.push([u, v, (u - v).abs()]);
                }
            }
        }
        assert_eq!(run(&times[..6000], 100, 1), 99);
    }
}
