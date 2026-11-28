// ==================================================================
// PRACTICE 0951: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0951_falling_squares.md
// Run the tests:          cargo test practice::p0951
// ==================================================================

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

pub struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
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
