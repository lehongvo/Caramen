// ==================================================================
// PRACTICE 0852: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0852_number_of_operations_to_make_network_connected.md
// Run the tests:          cargo test practice::p0852
// ==================================================================

//! # Problem 0852: Number of Operations to Make Network Connected
//!
//! - **Difficulty:** Medium
//! - **Topics:** union-find with a component counter, counting spare edges
//! - **Source:** [LeetCode 1319. Number of Operations to Make Network Connected](https://leetcode.com/problems/number-of-operations-to-make-network-connected/)
//!
//! ## Problem
//!
//! There are `n` computers `0..n` and some cables; `connections[i] = [a, b]` is a cable between `a`
//! and `b`. In one operation you may unplug any existing cable and use it to connect any two
//! computers that are not directly connected. Return the minimum number of operations needed so
//! that every computer can reach every other one (directly or indirectly), or `-1` if it is
//! impossible.
//!
//! Implement `Solution::make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, connections = [[0, 1], [0, 2], [1, 2]]
//! Output: 1
//! Explanation: move the cable 1-2 to connect 1 (or 2) with 3.
//!
//! Input:  n = 6, connections = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]
//! Output: 2
//!
//! Input:  n = 6, connections = [[0, 1], [0, 2], [0, 3], [1, 2]]
//! Output: -1
//! Explanation: 6 computers need at least 5 cables.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`
//! - `1 <= connections.len() <= min(n * (n - 1) / 2, 10^5)`
//! - `0 <= a, b < n`, `a != b`; no repeated connections.
//!
//! ## Hints
//!
//! 1. Connecting `n` computers needs at least `n - 1` cables. With fewer, return `-1`.
//! 2. With enough cables, every extra cable inside a component can be moved. If there are `k`
//!    components, `k - 1` moves join them.

pub struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, cables: &[[i32; 2]]) -> i32 {
        Solution::make_connected(n, cables.iter().map(|c| c.to_vec()).collect())
    }

    #[test]
    fn example_one_move() {
        assert_eq!(run(4, &[[0, 1], [0, 2], [1, 2]]), 1);
    }

    #[test]
    fn example_two_moves() {
        assert_eq!(run(6, &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]), 2);
    }

    #[test]
    fn example_not_enough_cables() {
        assert_eq!(run(6, &[[0, 1], [0, 2], [0, 3], [1, 2]]), -1);
    }

    #[test]
    fn already_connected() {
        assert_eq!(run(3, &[[0, 1], [1, 2]]), 0);
        assert_eq!(run(2, &[[0, 1]]), 0);
    }

    #[test]
    fn many_isolated_computers() {
        // A complete graph on 450 computers has 101 025 cables; use the first 100 000 of them,
        // which still connect those 450 computers. The other 99 550 are isolated.
        let mut cables = Vec::new();
        'outer: for a in 0..450 {
            for b in a + 1..450 {
                if cables.len() == 100_000 {
                    break 'outer;
                }
                cables.push([a, b]);
            }
        }
        assert_eq!(run(100_000, &cables), 100_000 - 450);
    }
}
