// ==================================================================
// PRACTICE 0393: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0393_k_closest_points_to_origin.md
// Run the tests:          cargo test practice::p0393
// ==================================================================

//! # Problem 0393: K Closest Points to Origin
//!
//! - **Difficulty:** Medium
//! - **Topics:** a max-heap of `(distance, index)` bounded to `k`, comparing squared distances, `into_iter` over a heap
//! - **Source:** [LeetCode 973. K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/)
//!
//! ## Problem
//!
//! You are given a list of points on the plane, `points[i] = [x, y]`, and an integer `k`. Return
//! the `k` points closest to the origin `(0, 0)` by Euclidean distance. The answer is unique up to
//! order, and it may be returned in **any order**.
//!
//! Implement `Solution::k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  points = [[1, 3], [-2, 2]], k = 1
//! Output: [[-2, 2]]       (distances sqrt(10) and sqrt(8))
//!
//! Input:  points = [[3, 3], [5, -1], [-2, 4]], k = 2
//! Output: [[3, 3], [-2, 4]]     ([[-2, 4], [3, 3]] is also accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= points.len() <= 10^4`
//! - `-10^4 <= x, y <= 10^4`
//!
//! ## Hints
//!
//! 1. Comparing `x^2 + y^2` gives the same order as comparing the real distances, with no square
//!    roots or floating point.
//! 2. Keep the `k` closest points seen so far in a **max**-heap keyed by distance; when it holds
//!    `k + 1` points, pop the farthest.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    fn example_two_any_order() {
        assert_eq!(
            sorted(Solution::k_closest(
                vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
                2
            )),
            vec![vec![-2, 4], vec![3, 3]]
        );
    }

    #[test]
    fn all_points_and_origin() {
        assert_eq!(
            sorted(Solution::k_closest(vec![vec![0, 0], vec![1, 1]], 2)),
            vec![vec![0, 0], vec![1, 1]]
        );
        assert_eq!(
            Solution::k_closest(vec![vec![10_000, -10_000], vec![0, 0]], 1),
            vec![vec![0, 0]]
        );
    }

    #[test]
    fn many_points_on_a_line() {
        // Points (i, 0) for i in -5000..5000 shuffled; the 3 closest are (-1, 0), (0, 0), (1, 0).
        let mut points: Vec<Vec<i32>> = (-5_000..5_000).map(|i| vec![i, 0]).collect();
        points.rotate_left(1_234);
        assert_eq!(
            sorted(Solution::k_closest(points, 3)),
            vec![vec![-1, 0], vec![0, 0], vec![1, 0]]
        );
    }
}
