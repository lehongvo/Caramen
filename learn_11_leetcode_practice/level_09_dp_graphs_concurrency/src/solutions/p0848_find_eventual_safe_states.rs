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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** iterative DFS. The stack holds `(node, index of the next edge to try)`. Looking at the
//! top node `u`: if it has another edge to `v`, then
//! - `v` new: mark it `OnPath` and push it;
//! - `v` on the path or unsafe: `u` is unsafe;
//! - `v` safe: nothing to do.
//!
//! When `u` has no edges left, pop it: if it is still `OnPath`, every successor was safe, so it is
//! `Safe`; if it is `Unsafe`, its parent (the new top) becomes `Unsafe` too.
//!
//! **Complexity:** `O(V + E)` time and `O(V)` space.
//!
//! **Rust concepts:**
//! - A small `#[derive(Clone, Copy, PartialEq)] enum State` documents the four states better than
//!   integers `0..=3`.
//! - `stack.last_mut()` gives `&mut (usize, usize)` to advance the edge index in place; the borrow
//!   ends before `stack.push` thanks to non-lexical lifetimes.
//! - An explicit stack avoids deep recursion on long chains (`n` can be `10^4`).
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Clone, Copy, PartialEq)]
enum State {
    New,
    OnPath,
    Safe,
    Unsafe,
}
// @solution-end

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        // @solution-start
        let n = graph.len();
        let mut state = vec![State::New; n];
        for start in 0..n {
            if state[start] != State::New {
                continue;
            }
            state[start] = State::OnPath;
            let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
            while let Some(top) = stack.last_mut() {
                let (u, edge) = *top;
                if let Some(&v) = graph[u].get(edge) {
                    top.1 += 1;
                    let v = v as usize;
                    match state[v] {
                        State::New => {
                            state[v] = State::OnPath;
                            stack.push((v, 0));
                        }
                        State::OnPath | State::Unsafe => state[u] = State::Unsafe,
                        State::Safe => {}
                    }
                } else {
                    stack.pop();
                    if state[u] == State::OnPath {
                        state[u] = State::Safe;
                    } else if let Some(&(parent, _)) = stack.last() {
                        state[parent] = State::Unsafe; // u is unsafe, so is its parent
                    }
                }
            }
        }
        (0..n)
            .filter(|&v| state[v] == State::Safe)
            .map(|v| v as i32)
            .collect()
        // @solution-end
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
