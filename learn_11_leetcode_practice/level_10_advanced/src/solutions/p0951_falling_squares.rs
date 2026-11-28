//! # Problem 0951: Falling Squares
//!
//! - **Difficulty:** Hard
//! - **Topics:** segment tree with lazy range assignment, coordinate compression, half-open intervals, recursion on `(node, lo, hi)`
//! - **Source:** [LeetCode 699. Falling Squares](https://leetcode.com/problems/falling-squares/)
//!
//! ## Problem
//!
//! Squares are dropped one after another onto the X axis. `positions[i] = [left, side]` drops a square
//! with side length `side` whose left edge is at `x = left`; it covers the half-open range
//! `[left, left + side)`. It falls straight down until it lands on the ground or on top of a
//! previously dropped square that overlaps it with **positive** length (touching at an edge does not
//! count), and then it stays there.
//!
//! After each drop, record the height of the **tallest stack** so far. Implement
//! `Solution::falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  positions = [[1, 2], [2, 3], [6, 1]]
//! Output: [2, 5, 5]
//! (The first square reaches height 2; the second lands on it and reaches 2 + 3 = 5;
//!  the third lands on the ground at height 1, so the maximum stays 5.)
//!
//! Input:  positions = [[100, 100], [200, 100]]
//! Output: [100, 100]       (they only touch at x = 200)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= positions.len() <= 1000`
//! - `1 <= left <= 10^8`, `1 <= side <= 10^6`
//!
//! ## Hints
//!
//! 1. Only the `2n` edges matter: sort and deduplicate them, so elementary segment `k` is
//!    `[xs[k], xs[k + 1])`.
//! 2. A square needs the **maximum** height over its segments, then **sets** all of them to that
//!    maximum plus its side: a segment tree with "range max" and "range assign" does both in
//!    `O(log n)`.
//! 3. Lazy propagation: store a pending assignment in a node and push it to the children before
//!    visiting them.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** compress the coordinates, then keep a segment tree over the elementary segments. For each
//! square, query the maximum height on its segment range, add the side, and assign the result to the
//! whole range. Assignment is lazy: a node fully covered by the range stores the new value both as
//! its maximum and as a pending tag for its children. The answer after each drop is the running
//! maximum of the new heights.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `binary_search` on the sorted, deduplicated coordinate vector maps a coordinate to its index.
//! - The tree stores `Vec<i32>` maxima and `Vec<Option<i32>>` pending assignments; `Option::take`
//!   consumes a tag when it is pushed down.
//! - Recursive methods take `&mut self` and the node's half-open range `[lo, hi)`.
// @solution-end

pub struct Solution;

// @solution-start remove
struct SegmentTree {
    max: Vec<i32>,
    pending: Vec<Option<i32>>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        SegmentTree {
            max: vec![0; 4 * size.max(1)],
            pending: vec![None; 4 * size.max(1)],
        }
    }

    fn push_down(&mut self, node: usize) {
        if let Some(value) = self.pending[node].take() {
            for child in [2 * node, 2 * node + 1] {
                self.max[child] = value;
                self.pending[child] = Some(value);
            }
        }
    }

    /// Maximum over `[l, r)`, for the node covering `[lo, hi)`.
    fn query(&mut self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i32 {
        if r <= lo || hi <= l {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.max[node];
        }
        self.push_down(node);
        let mid = (lo + hi) / 2;
        self.query(2 * node, lo, mid, l, r)
            .max(self.query(2 * node + 1, mid, hi, l, r))
    }

    /// Sets every position of `[l, r)` to `value`.
    fn assign(&mut self, node: usize, lo: usize, hi: usize, l: usize, r: usize, value: i32) {
        if r <= lo || hi <= l {
            return;
        }
        if l <= lo && hi <= r {
            self.max[node] = value;
            self.pending[node] = Some(value);
            return;
        }
        self.push_down(node);
        let mid = (lo + hi) / 2;
        self.assign(2 * node, lo, mid, l, r, value);
        self.assign(2 * node + 1, mid, hi, l, r, value);
        self.max[node] = self.max[2 * node].max(self.max[2 * node + 1]);
    }
}
// @solution-end

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        // @solution-start
        let mut xs: Vec<i64> = positions
            .iter()
            .flat_map(|p| [i64::from(p[0]), i64::from(p[0]) + i64::from(p[1])])
            .collect();
        xs.sort_unstable();
        xs.dedup();
        let segments = xs.len() - 1;
        let index = |x: i64| xs.binary_search(&x).unwrap_or_else(|i| i);
        let mut tree = SegmentTree::new(segments);
        let mut best = 0;
        let mut answer = Vec::with_capacity(positions.len());
        for p in &positions {
            let (left, side) = (i64::from(p[0]), p[1]);
            let (l, r) = (index(left), index(left + i64::from(side)));
            let height = tree.query(1, 0, segments, l, r) + side;
            tree.assign(1, 0, segments, l, r, height);
            best = best.max(height);
            answer.push(best);
        }
        answer
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(positions: &[[i32; 2]]) -> Vec<i32> {
        Solution::falling_squares(positions.iter().map(|p| p.to_vec()).collect())
    }

    fn naive(positions: &[[i32; 2]]) -> Vec<i32> {
        let mut placed: Vec<(i64, i64, i32)> = Vec::new();
        let mut best = 0;
        positions
            .iter()
            .map(|&[left, side]| {
                let (l, r) = (i64::from(left), i64::from(left) + i64::from(side));
                let base = placed
                    .iter()
                    .filter(|&&(pl, pr, _)| pl < r && l < pr)
                    .map(|&(_, _, h)| h)
                    .max()
                    .unwrap_or(0);
                placed.push((l, r, base + side));
                best = best.max(base + side);
                best
            })
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(run(&[[1, 2], [2, 3], [6, 1]]), vec![2, 5, 5]);
        assert_eq!(run(&[[100, 100], [200, 100]]), vec![100, 100]);
    }

    #[test]
    fn stacking_exactly_on_top() {
        assert_eq!(run(&[[1, 5], [1, 5], [1, 5]]), vec![5, 10, 15]);
        assert_eq!(
            run(&[[1, 10], [5, 1], [3, 2], [20, 30]]),
            vec![10, 11, 12, 30]
        );
    }

    #[test]
    fn matches_naive_simulation() {
        let positions: Vec<[i32; 2]> = (0..300)
            .map(|i: i32| [(i * 37) % 200 + 1, (i * 13) % 17 + 1])
            .collect();
        assert_eq!(run(&positions), naive(&positions));
    }

    #[test]
    fn large_coordinates() {
        let positions: Vec<[i32; 2]> = (0..1_000)
            .map(|i: i32| [100_000_000 - i * 50_000, 1_000_000])
            .collect();
        let result = run(&positions);
        assert_eq!(result.len(), 1_000);
        assert_eq!(result, naive(&positions));
    }
}
