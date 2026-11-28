//! # Problem 0851: Redundant Connection
//!
//! - **Difficulty:** Medium
//! - **Topics:** union-find (disjoint set union) as a small struct with methods, path halving,
//!   union by size
//! - **Source:** [LeetCode 684. Redundant Connection](https://leetcode.com/problems/redundant-connection/)
//!
//! ## Problem
//!
//! A tree with nodes `1..=n` had **one extra edge** added, so the graph now has `n` undirected
//! edges and exactly one cycle. `edges[i] = [a, b]` lists them in a given order. Return an edge
//! that can be removed so the remaining edges form a tree again. If several edges qualify (every
//! edge on the cycle does), return the one that appears **last** in `edges`.
//!
//! Implement `Solution::find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  edges = [[1, 2], [1, 3], [2, 3]]
//! Output: [2, 3]
//!
//! Input:  edges = [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]
//! Output: [1, 4]
//! Explanation: the cycle is 1-2-3-4-1; [1, 4] is its last edge in the input.
//! ```
//!
//! ## Constraints
//!
//! - `3 <= n == edges.len() <= 1000`
//! - `1 <= a < b <= n`; no repeated edges; the graph is connected.
//!
//! ## Hints
//!
//! 1. Add the edges one by one. The first edge whose two ends are **already** connected closes the
//!    cycle, and it is the last cycle edge in input order.
//! 2. "Are `a` and `b` already connected?" is the question a union-find structure answers in
//!    almost constant time.
//! 3. `find(x)` follows parent links to the representative of `x`'s set; `union(a, b)` links two
//!    representatives and reports whether they were different.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a `DisjointSets` struct with `parent` and `size` vectors. `find` walks up to the root,
//! halving the path on the way (each visited node is re-linked to its grandparent). `union` finds
//! both roots; if they are equal the edge is redundant, otherwise the smaller tree is attached under
//! the larger one. Return the first edge for which `union` reports "already connected".
//!
//! **Complexity:** `O(n * alpha(n))` time (`alpha` is the inverse Ackermann function, at most 4 in
//! practice), `O(n)` space.
//!
//! **Rust concepts:**
//! - A private struct with `impl` methods (`new`, `find(&mut self)`, `union(&mut self) -> bool`)
//!   keeps the union-find logic reusable and testable; problems 0852-0855 use the same shape.
//! - `find` mutates (path halving) so it needs `&mut self`, even though it is a "query".
//! - `std::mem::swap` on the two roots makes "attach the smaller under the larger" one branch.
// @solution-end

pub struct Solution;

// @solution-start remove
struct DisjointSets {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSets {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // path halving
            x = self.parent[x];
        }
        x
    }

    /// Joins the sets of `a` and `b`; returns `false` if they were already the same set.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}
// @solution-end

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        // @solution-start
        let mut sets = DisjointSets::new(edges.len() + 1);
        edges
            .into_iter()
            .find(|e| !sets.union(e[0] as usize, e[1] as usize))
            .unwrap_or_default() // the input always contains exactly one redundant edge
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(edges: &[[i32; 2]]) -> Vec<i32> {
        Solution::find_redundant_connection(edges.iter().map(|e| e.to_vec()).collect())
    }

    #[test]
    fn example_triangle() {
        assert_eq!(run(&[[1, 2], [1, 3], [2, 3]]), vec![2, 3]);
    }

    #[test]
    fn example_square_with_tail() {
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]), vec![1, 4]);
    }

    #[test]
    fn cycle_edge_in_the_middle() {
        assert_eq!(run(&[[1, 2], [2, 3], [1, 3], [3, 4]]), vec![1, 3]);
    }

    #[test]
    fn large_ring() {
        let mut edges: Vec<[i32; 2]> = (1..1000).map(|i| [i, i + 1]).collect();
        edges.push([1, 1000]);
        assert_eq!(run(&edges), vec![1, 1000]);
    }

    #[test]
    fn ring_closed_early() {
        // The cycle 1-2-4-3-1 closes with [1, 3] before the tail edges are added.
        let edges = [[3, 4], [1, 2], [2, 4], [1, 3], [4, 5], [5, 6]];
        assert_eq!(run(&edges), vec![1, 3]);
    }
}
