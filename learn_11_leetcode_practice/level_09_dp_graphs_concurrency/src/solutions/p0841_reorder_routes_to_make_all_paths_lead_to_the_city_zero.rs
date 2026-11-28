//! # Problem 0841: Reorder Routes to Make All Paths Lead to the City Zero
//!
//! - **Difficulty:** Medium
//! - **Topics:** trees as graphs, adjacency list with edge flags `Vec<Vec<(usize, bool)>>`,
//!   iterative DFS
//! - **Source:** [LeetCode 1466. Reorder Routes to Make All Paths Lead to the City Zero](https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/)
//!
//! ## Problem
//!
//! There are `n` cities `0..n` and exactly `n - 1` one-way roads; `connections[i] = [a, b]` is a
//! road from `a` to `b`. Ignoring directions, the roads form a tree (every city can be reached
//! from every other one in exactly one way). The capital is city `0`. Return the minimum number of
//! roads whose direction must be reversed so that every city can drive to city `0`.
//!
//! Implement `Solution::min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 6, connections = [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]
//! Output: 3
//! Explanation: reverse 0->1, 1->3 and 4->5.
//!
//! Input:  n = 5, connections = [[1, 0], [1, 2], [3, 2], [3, 4]]
//! Output: 2
//! Explanation: reverse 1->2 and 3->4.
//!
//! Input:  n = 3, connections = [[1, 0], [2, 0]]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 5 * 10^4`
//! - `connections.len() == n - 1`, `connections[i].len() == 2`
//! - `0 <= a, b < n`, `a != b`
//!
//! ## Hints
//!
//! 1. Walk the tree outward from city `0`, ignoring directions. Every road you cross is crossed
//!    away from the capital.
//! 2. A road that already points away from `0` (the same way you crossed it) must be reversed; a
//!    road pointing toward `0` is fine.
//! 3. Store, for each neighbour, whether the original road goes "out" from the current city.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for each road `a -> b`, add `(b, true)` to `adj[a]` (crossing from `a` to `b` follows
//! the road) and `(a, false)` to `adj[b]`. DFS from `0`; every time you move to an unvisited
//! neighbour through an edge marked `true`, that road points away from the capital and must be
//! reversed.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - An adjacency list of tuples `(neighbour, flag)` keeps the extra edge information next to the
//!   neighbour.
//! - In a tree, "visited" can be replaced by remembering the parent, but a `Vec<bool>` is simpler
//!   with an explicit stack.
//! - `bool as i32` converts `true`/`false` to `1`/`0` for counting.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, bool)>> = vec![Vec::new(); n];
        for c in &connections {
            let (a, b) = (c[0] as usize, c[1] as usize);
            adj[a].push((b, true));
            adj[b].push((a, false));
        }
        let mut seen = vec![false; n];
        seen[0] = true;
        let mut stack = vec![0];
        let mut reversed = 0;
        while let Some(u) = stack.pop() {
            for &(v, points_away) in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    reversed += points_away as i32;
                    stack.push(v);
                }
            }
        }
        reversed
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, roads: &[[i32; 2]]) -> i32 {
        Solution::min_reorder(n, roads.iter().map(|r| r.to_vec()).collect())
    }

    #[test]
    fn example_six_cities() {
        assert_eq!(run(6, &[[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]), 3);
    }

    #[test]
    fn example_five_cities() {
        assert_eq!(run(5, &[[1, 0], [1, 2], [3, 2], [3, 4]]), 2);
    }

    #[test]
    fn example_already_fine() {
        assert_eq!(run(3, &[[1, 0], [2, 0]]), 0);
    }

    #[test]
    fn two_cities() {
        assert_eq!(run(2, &[[0, 1]]), 1);
        assert_eq!(run(2, &[[1, 0]]), 0);
    }

    #[test]
    fn long_chain_pointing_away() {
        let n = 50_000;
        let roads: Vec<[i32; 2]> = (0..n - 1).map(|i| [i, i + 1]).collect();
        assert_eq!(run(n, &roads), n - 1);
    }
}
