//! # Problem 0860: Number of Ways to Arrive at Destination
//!
//! - **Difficulty:** Medium
//! - **Topics:** Dijkstra that also counts shortest paths, `i64` distances, counts modulo
//!   `10^9 + 7`, `std::cmp::Ordering`
//! - **Source:** [LeetCode 1976. Number of Ways to Arrive at Destination](https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/)
//!
//! ## Problem
//!
//! A city has `n` intersections `0..n` and bidirectional roads; `roads[i] = [u, v, time]` takes
//! `time` minutes in either direction. Every intersection is reachable from every other one, and
//! there is at most one road between two intersections. Return the number of different routes from
//! intersection `0` to intersection `n - 1` that take the **shortest possible** total time. The
//! number can be huge, so return it modulo `10^9 + 7`.
//!
//! Implement `Solution::count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 7, roads = [[0, 6, 7], [0, 1, 2], [1, 2, 3], [1, 3, 3], [6, 3, 3], [3, 5, 1],
//!                         [6, 5, 1], [2, 5, 1], [0, 4, 5], [4, 6, 2]]
//! Output: 4
//! Explanation: the shortest time is 7, achieved by 0-6, 0-4-6, 0-1-2-5-6 and 0-1-3-5-6.
//!
//! Input:  n = 2, roads = [[1, 0, 10]]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 200`; `n - 1 <= roads.len() <= n * (n - 1) / 2`
//! - `1 <= time <= 10^9`, so a total time can exceed `i32::MAX`.
//!
//! ## Hints
//!
//! 1. Run Dijkstra from `0`, and keep `ways[v]`: the number of shortest routes to `v` found so far.
//! 2. When relaxing `u -> v`: a strictly shorter distance replaces `ways[v]` with `ways[u]`; an
//!    equal distance adds `ways[u]` to `ways[v]`.
//! 3. Because a node is settled only after all nodes with smaller distance, `ways[u]` is final when
//!    `u` is popped.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** Dijkstra with `dist: Vec<i64>` and `ways: Vec<i64>`. For every non-stale popped node
//! `u` and edge `(v, t)`, compare `dist[u] + t` with `dist[v]` using `cmp`: `Less` resets
//! `ways[v] = ways[u]` and pushes `v`; `Equal` adds `ways[u]` modulo `10^9 + 7`; `Greater` does
//! nothing.
//!
//! **Complexity:** `O(E log E)` time, `O(V + E)` space.
//!
//! **Rust concepts:**
//! - Times up to `10^9` along up to 199 roads need `i64`; `i64::from(t)` widens losslessly.
//! - `match a.cmp(&b) { Ordering::Less => .., Ordering::Equal => .., Ordering::Greater => {} }`
//!   handles the three relaxation outcomes exhaustively.
//! - `const MOD: i64 = 1_000_000_007;` inside the function keeps the constant local.
// @solution-end

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
        for r in &roads {
            let (u, v, t) = (r[0] as usize, r[1] as usize, i64::from(r[2]));
            adj[u].push((v, t));
            adj[v].push((u, t));
        }
        let mut dist = vec![i64::MAX; n];
        let mut ways = vec![0_i64; n];
        dist[0] = 0;
        ways[0] = 1;
        let mut heap = BinaryHeap::from([Reverse((0_i64, 0_usize))]);
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, t) in &adj[u] {
                let candidate = d + t;
                match candidate.cmp(&dist[v]) {
                    Ordering::Less => {
                        dist[v] = candidate;
                        ways[v] = ways[u];
                        heap.push(Reverse((candidate, v)));
                    }
                    Ordering::Equal => ways[v] = (ways[v] + ways[u]) % MOD,
                    Ordering::Greater => {}
                }
            }
        }
        ways[n - 1] as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, roads: &[[i32; 3]]) -> i32 {
        Solution::count_paths(n, roads.iter().map(|r| r.to_vec()).collect())
    }

    #[test]
    fn example_four_routes() {
        let roads = [
            [0, 6, 7],
            [0, 1, 2],
            [1, 2, 3],
            [1, 3, 3],
            [6, 3, 3],
            [3, 5, 1],
            [6, 5, 1],
            [2, 5, 1],
            [0, 4, 5],
            [4, 6, 2],
        ];
        assert_eq!(run(7, &roads), 4);
    }

    #[test]
    fn example_single_road() {
        assert_eq!(run(2, &[[1, 0, 10]]), 1);
    }

    #[test]
    fn single_intersection() {
        assert_eq!(run(1, &[]), 1);
    }

    #[test]
    fn longer_road_is_not_counted() {
        // 0-1-2 takes 2, the direct road 0-2 takes 3.
        assert_eq!(run(3, &[[0, 1, 1], [1, 2, 1], [0, 2, 3]]), 1);
        assert_eq!(run(3, &[[0, 1, 1], [1, 2, 1], [0, 2, 2]]), 2);
    }

    #[test]
    fn forty_diamonds_need_modulo_and_i64() {
        // Each diamond s -> {a, b} -> e offers 2 equal routes; 40 diamonds in a row give 2^40.
        let mut roads = Vec::new();
        for i in 0..40 {
            let s = 3 * i;
            let (a, b, e) = (s + 1, s + 2, s + 3);
            let t = 1_000_000_000;
            roads.extend([[s, a, t], [s, b, t], [a, e, t], [b, e, t]]);
        }
        // 2^40 mod (10^9 + 7)
        assert_eq!(run(121, &roads), 511_620_083);
    }
}
