//! # Problem 0999: Trapping Rain Water II
//!
//! - **Difficulty:** Hard
//! - **Topics:** priority-first flood fill from the border, `BinaryHeap<Reverse<(i32, usize, usize)>>`, visited grid, water level = running max
//! - **Source:** [LeetCode 407. Trapping Rain Water II](https://leetcode.com/problems/trapping-rain-water-ii/)
//!
//! ## Problem
//!
//! `height_map[r][c]` is the height of a unit cell of an elevation map. After heavy rain, water is
//! trapped wherever it cannot flow off the map; water flows between horizontally or vertically
//! adjacent cells, and off the map from border cells. Return the total volume of trapped water.
//!
//! Implement `Solution::trap_rain_water(height_map: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! height_map = [[1, 4, 3, 1, 3, 2],
//!               [3, 2, 1, 3, 2, 4],
//!               [2, 3, 3, 2, 3, 1]]          => 4
//!
//! height_map = [[3, 3, 3, 3, 3],
//!               [3, 2, 2, 2, 3],
//!               [3, 2, 1, 2, 3],
//!               [3, 2, 2, 2, 3],
//!               [3, 3, 3, 3, 3]]             => 10
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`, `0 <= height_map[r][c] <= 2 * 10^4`
//!
//! ## Hints
//!
//! 1. The border cells can never hold water; they form the initial "wall".
//! 2. Always extend the wall at its **lowest** cell (min-heap): water entering its unvisited neighbour
//!    can rise up to that wall height, and no lower path to the outside exists.
//! 3. A neighbour lower than the current level traps `level - height`; push it with height
//!    `max(level, height)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a Dijkstra-like flood from the outside. The heap holds the boundary of the visited area
//! keyed by its effective height (the water level water would reach there). Popping the lowest
//! boundary cell, every unvisited neighbour is enclosed by walls at least that high on the processed
//! side, so it fills up to `max(level, its height)`, contributing the difference.
//!
//! **Complexity:** `O(m n log(m n))` time, `O(m n)` space.
//!
//! **Rust concepts:**
//! - `Reverse` turns `BinaryHeap` into a min-heap; tuples compare by height first.
//! - Neighbour coordinates are produced with `checked_sub` and bounds checks, without signed casts.
// @solution-end

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let (m, n) = (height_map.len(), height_map[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut heap = BinaryHeap::new();
        for r in 0..m {
            for c in 0..n {
                if r == 0 || c == 0 || r == m - 1 || c == n - 1 {
                    visited[r][c] = true;
                    heap.push(Reverse((height_map[r][c], r, c)));
                }
            }
        }
        let mut water = 0;
        while let Some(Reverse((level, r, c))) = heap.pop() {
            let candidates = [
                r.checked_sub(1).map(|nr| (nr, c)),
                (r + 1 < m).then_some((r + 1, c)),
                c.checked_sub(1).map(|nc| (r, nc)),
                (c + 1 < n).then_some((r, c + 1)),
            ];
            for (nr, nc) in candidates.into_iter().flatten() {
                if visited[nr][nc] {
                    continue;
                }
                visited[nr][nc] = true;
                let h = height_map[nr][nc];
                water += (level - h).max(0);
                heap.push(Reverse((level.max(h), nr, nc)));
            }
        }
        water
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        assert_eq!(Solution::trap_rain_water(map), 4);
        let bowl = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        assert_eq!(Solution::trap_rain_water(bowl), 10);
    }

    #[test]
    fn thin_maps_hold_nothing() {
        assert_eq!(Solution::trap_rain_water(vec![vec![5, 1, 5]]), 0);
        assert_eq!(
            Solution::trap_rain_water(vec![vec![5], vec![1], vec![5]]),
            0
        );
        assert_eq!(Solution::trap_rain_water(vec![vec![1, 1], vec![1, 1]]), 0);
    }

    #[test]
    fn leaks_through_the_lowest_gap() {
        // The wall has a gap of height 1, so the pit only fills up to 1.
        let map = vec![
            vec![5, 5, 5, 5],
            vec![5, 0, 0, 1],
            vec![5, 0, 0, 5],
            vec![5, 5, 5, 5],
        ];
        assert_eq!(Solution::trap_rain_water(map), 4);
    }

    #[test]
    fn large_pool() {
        let n = 200;
        let map: Vec<Vec<i32>> = (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| {
                        if r == 0 || c == 0 || r == n - 1 || c == n - 1 {
                            20_000
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::trap_rain_water(map), 198 * 198 * 20_000);
    }
}
