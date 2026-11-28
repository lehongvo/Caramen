//! # Problem 0961: Kth Ancestor of a Tree Node
//!
//! - **Difficulty:** Hard
//! - **Topics:** binary lifting (jump tables of `2^j` ancestors), `Vec<Vec<i32>>` tables, bit iteration of `k`
//! - **Source:** [LeetCode 1483. Kth Ancestor of a Tree Node](https://leetcode.com/problems/kth-ancestor-of-a-tree-node/)
//!
//! ## Problem
//!
//! A rooted tree has `n` nodes numbered `0..n`; `parent[i]` is the parent of node `i`, and
//! `parent[0] == -1` because node `0` is the root. Answer many queries "what is the `k`-th ancestor
//! of `node`?": the 1st ancestor is the parent, the 2nd is the parent's parent, and so on. If the
//! ancestor does not exist, the answer is `-1`.
//!
//! Implement the `TreeAncestor` struct:
//!
//! - `TreeAncestor::new(n: i32, parent: Vec<i32>) -> Self`.
//! - `get_kth_ancestor(&self, node: i32, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! TreeAncestor::new(7, [-1, 0, 0, 1, 1, 2, 2])
//! get_kth_ancestor(3, 1) => 1
//! get_kth_ancestor(5, 2) => 0
//! get_kth_ancestor(6, 3) => -1      (node 6 is only at depth 2)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= n <= 5 * 10^4`
//! - `parent[i] < i` for every `i > 0` (parents have smaller numbers), `parent[0] == -1`.
//! - At most `5 * 10^4` queries.
//!
//! ## Hints
//!
//! 1. Walking up one parent at a time costs `O(k)` per query: too slow for a path-shaped tree.
//! 2. Precompute `up[j][v]`, the `2^j`-th ancestor of `v`: `up[0] = parent` and
//!    `up[j][v] = up[j - 1][up[j - 1][v]]` (or `-1`).
//! 3. Write `k` in binary and jump by `2^j` for every set bit `j`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary lifting. The table has `LOG = 16` levels (`2^16 > 5 * 10^4`). Each query jumps
//! once per set bit of `k`, stopping early when it runs past the root.
//!
//! **Complexity:** `O(n log n)` preprocessing and space, `O(log k)` per query.
//!
//! **Rust concepts:**
//! - Building each level from the previous one with `iter().map(...).collect()` keeps the table
//!   construction declarative.
//! - `-1` is LeetCode's "no node" value; converting with `usize::try_from` turns it into an error
//!   that `?`-style code (or `ok()`) can handle.
//! - `(k >> j) & 1` tests bit `j` of `k`.
// @solution-end

pub struct TreeAncestor {
    // @solution-start remove
    up: Vec<Vec<i32>>,
    // @solution-end
}

// @solution-start remove
const LOG: usize = 16;
// @solution-end

impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        // @solution-start
        debug_assert_eq!(parent.len(), n as usize);
        let mut up = vec![parent];
        for j in 1..LOG {
            let previous = &up[j - 1];
            let level: Vec<i32> = previous
                .iter()
                .map(|&p| match usize::try_from(p) {
                    Ok(p) => previous[p],
                    Err(_) => -1,
                })
                .collect();
            up.push(level);
        }
        TreeAncestor { up }
        // @solution-end
    }

    pub fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        // @solution-start
        let mut current = node;
        for j in 0..LOG {
            if (k >> j) & 1 == 1 {
                let Ok(index) = usize::try_from(current) else {
                    return -1;
                };
                current = self.up[j][index];
            }
        }
        if k >> LOG != 0 { -1 } else { current }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let t = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(t.get_kth_ancestor(3, 1), 1);
        assert_eq!(t.get_kth_ancestor(5, 2), 0);
        assert_eq!(t.get_kth_ancestor(6, 3), -1);
    }

    #[test]
    fn root_and_single_node() {
        let t = TreeAncestor::new(1, vec![-1]);
        assert_eq!(t.get_kth_ancestor(0, 1), -1);
        let t = TreeAncestor::new(3, vec![-1, 0, 1]);
        assert_eq!(t.get_kth_ancestor(2, 2), 0);
        assert_eq!(t.get_kth_ancestor(2, 3), -1);
    }

    #[test]
    fn long_path() {
        let n = 50_000;
        let parent: Vec<i32> = (0..n).map(|i| i - 1).collect();
        let t = TreeAncestor::new(n, parent);
        assert_eq!(t.get_kth_ancestor(n - 1, n - 1), 0);
        assert_eq!(t.get_kth_ancestor(n - 1, n), -1);
        assert_eq!(t.get_kth_ancestor(40_000, 12_345), 27_655);
        for k in 1..=1_000 {
            assert_eq!(t.get_kth_ancestor(49_999, k), 49_999 - k);
        }
    }

    #[test]
    fn matches_walking_up() {
        let parent: Vec<i32> = (0..500)
            .map(|i: i32| if i == 0 { -1 } else { (i * 7 + 3) % i })
            .collect();
        let t = TreeAncestor::new(500, parent.clone());
        for node in 0..500 {
            for k in 1..20 {
                let mut expected = node;
                for _ in 0..k {
                    if expected != -1 {
                        expected = parent[expected as usize];
                    }
                }
                assert_eq!(t.get_kth_ancestor(node, k), expected, "node {node}, k {k}");
            }
        }
    }
}
