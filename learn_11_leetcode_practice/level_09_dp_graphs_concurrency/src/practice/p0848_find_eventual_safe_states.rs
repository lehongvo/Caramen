// ==================================================================
// PRACTICE 0848: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0848_find_eventual_safe_states.md
// Run the tests:          cargo test practice::p0848
// ==================================================================

//! # Problem 0848: Find Eventual Safe States
//!
//! - **Difficulty:** Medium
//! - **Topics:** three-color DFS with a private `enum`, iterative DFS with an explicit
//!   `(node, next edge)` stack, `last_mut`
//! - **Source:** [LeetCode 802. Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states/)
//!
//! ## Problem
//!
//! A directed graph with `n` nodes `0..n` is given as an adjacency list: `graph[i]` lists the nodes
//! `j` with an edge `i -> j`. A node is **terminal** if it has no outgoing edge. A node is **safe**
//! if every path that starts at it ends at a terminal node (in other words, no path from it can run
//! forever around a cycle). Return all safe nodes in increasing order.
//!
//! Implement `Solution::eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  graph = [[1, 2], [2, 3], [5], [0], [5], [], []]
//! Output: [2, 4, 5, 6]
//! Explanation: 5 and 6 are terminal; 2 and 4 only lead to 5. Nodes 0, 1, 3 lie on the cycle
//! 0 -> 1 -> 3 -> 0.
//!
//! Input:  graph = [[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []]
//! Output: [4]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`
//! - `0 <= graph[i].len() <= n`, values in `0..n`, sorted and distinct; self-loops are possible.
//! - The total number of edges is at most `4 * 10^4`.
//!
//! ## Hints
//!
//! 1. A node is unsafe exactly when it can reach a cycle.
//! 2. During a DFS, give each node a state: not visited, on the current path, known safe, or known
//!    unsafe. Meeting a node that is "on the current path" means you found a cycle.
//! 3. A node is safe if all of its successors are safe; unsafety propagates back to every node on
//!    the current path.

pub struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_seven_nodes() {
        let g = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        assert_eq!(Solution::eventual_safe_nodes(g), vec![2, 4, 5, 6]);
    }

    #[test]
    fn example_only_terminal_is_safe() {
        let g = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(g), vec![4]);
    }

    #[test]
    fn self_loop_and_lonely_node() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![vec![0]]),
            Vec::<i32>::new()
        );
        assert_eq!(Solution::eventual_safe_nodes(vec![vec![]]), vec![0]);
    }

    #[test]
    fn node_reaching_a_cycle_later() {
        // 0 -> 1 -> 2 -> 3 -> 2 (cycle), and 0 -> 4 (terminal). 0 is unsafe because of 1.
        let g = vec![vec![4, 1], vec![2], vec![3], vec![2], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(g), vec![4]);
    }

    #[test]
    fn long_chain_with_and_without_a_loop_at_the_end() {
        let n = 10_000;
        let mut g: Vec<Vec<i32>> = (0..n).map(|i| vec![i + 1]).collect();
        g[n as usize - 1] = vec![];
        assert_eq!(Solution::eventual_safe_nodes(g.clone()).len(), n as usize);
        g[n as usize - 1] = vec![n - 1];
        assert!(Solution::eventual_safe_nodes(g).is_empty());
    }
}
