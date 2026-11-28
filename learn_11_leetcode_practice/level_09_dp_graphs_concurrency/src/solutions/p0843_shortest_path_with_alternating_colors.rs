//! # Problem 0843: Shortest Path with Alternating Colors
//!
//! - **Difficulty:** Medium
//! - **Topics:** BFS over an expanded state space `(node, last color)`, `[Option<i32>; 2]` per node
//! - **Source:** [LeetCode 1129. Shortest Path with Alternating Colors](https://leetcode.com/problems/shortest-path-with-alternating-colors/)
//!
//! ## Problem
//!
//! A directed graph has `n` nodes `0..n`. Each edge is red or blue: `red_edges[i] = [a, b]` is a
//! red edge `a -> b` and `blue_edges[j] = [u, v]` a blue edge `u -> v`. There may be self-loops and
//! parallel edges.
//!
//! Return a vector `answer` of length `n` where `answer[x]` is the length (number of edges) of the
//! shortest path from node `0` to node `x` whose edge colors **alternate** (red, blue, red, ... or
//! blue, red, blue, ...), or `-1` if no such path exists. `answer[0]` is `0`.
//!
//! Implement `Solution::shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 3, red_edges = [[0, 1], [1, 2]], blue_edges = []
//! Output: [0, 1, -1]
//! Explanation: 0 -> 1 -> 2 would use two red edges in a row.
//!
//! Input:  n = 3, red_edges = [[0, 1]], blue_edges = [[1, 2]]
//! Output: [0, 1, 2]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 100`
//! - `0 <= red_edges.len(), blue_edges.len() <= 400`
//! - `0 <= a, b, u, v < n`
//!
//! ## Hints
//!
//! 1. Reaching a node by a red edge is not the same situation as reaching it by a blue edge: the
//!    next allowed color differs. Make the BFS state the pair `(node, color of the last edge)`.
//! 2. Start BFS from both `(0, red)` and `(0, blue)` at distance `0`.
//! 3. The answer for a node is the smaller of its two state distances.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build one adjacency list per color: `adj[color][u]`. BFS over states `(u, c)` where
//! `c` is the color of the edge used to arrive. From `(u, c)` follow only edges of color `1 - c`.
//! Each state is visited once, so BFS distances are shortest. Combine the two distances per node.
//!
//! **Complexity:** `O(n + e)` time and space (there are `2n` states).
//!
//! **Rust concepts:**
//! - `[Vec<Vec<usize>>; 2]` indexed by the color keeps the two graphs side by side.
//! - `dist: Vec<[Option<i32>; 2]>` stores the distance of both states of each node;
//!   `.into_iter().flatten().min()` picks the best existing one.
// @solution-end

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // @solution-start
        let n = n as usize;
        let mut adj: [Vec<Vec<usize>>; 2] = [vec![Vec::new(); n], vec![Vec::new(); n]];
        for (color, edges) in [red_edges, blue_edges].iter().enumerate() {
            for e in edges {
                adj[color][e[0] as usize].push(e[1] as usize);
            }
        }
        let mut dist: Vec<[Option<i32>; 2]> = vec![[None, None]; n];
        dist[0] = [Some(0), Some(0)];
        let mut queue = VecDeque::from([(0_usize, 0_usize), (0, 1)]);
        while let Some((u, c)) = queue.pop_front() {
            let d = dist[u][c].unwrap_or(0); // every queued state has a distance
            let next = 1 - c;
            for &v in &adj[next][u] {
                if dist[v][next].is_none() {
                    dist[v][next] = Some(d + 1);
                    queue.push_back((v, next));
                }
            }
        }
        dist.into_iter()
            .map(|pair| pair.into_iter().flatten().min().unwrap_or(-1))
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, red: &[[i32; 2]], blue: &[[i32; 2]]) -> Vec<i32> {
        let to_vec = |list: &[[i32; 2]]| list.iter().map(|e| e.to_vec()).collect();
        Solution::shortest_alternating_paths(n, to_vec(red), to_vec(blue))
    }

    #[test]
    fn example_two_reds() {
        assert_eq!(run(3, &[[0, 1], [1, 2]], &[]), vec![0, 1, -1]);
    }

    #[test]
    fn example_red_then_blue() {
        assert_eq!(run(3, &[[0, 1]], &[[1, 2]]), vec![0, 1, 2]);
    }

    #[test]
    fn edges_pointing_the_wrong_way() {
        assert_eq!(run(3, &[[0, 1]], &[[2, 1]]), vec![0, 1, -1]);
        assert_eq!(run(3, &[[0, 1], [0, 2]], &[[1, 0]]), vec![0, 1, 1]);
    }

    #[test]
    fn single_node() {
        assert_eq!(run(1, &[], &[]), vec![0]);
    }

    #[test]
    fn detour_through_a_cycle() {
        // Node 4 is reached only after looping 1 -> 2 -> 3 -> 1 once to fix the color parity.
        let red = [[0, 1], [1, 2], [2, 3], [3, 4]];
        let blue = [[1, 2], [2, 3], [3, 1]];
        assert_eq!(run(5, &red, &blue), vec![0, 1, 2, 3, 7]);
    }

    #[test]
    fn self_loops_are_allowed() {
        let red = [[0, 0], [0, 1], [2, 3]];
        let blue = [[0, 0], [1, 2], [3, 3]];
        assert_eq!(run(4, &red, &blue), vec![0, 1, 2, 3]);
    }
}
