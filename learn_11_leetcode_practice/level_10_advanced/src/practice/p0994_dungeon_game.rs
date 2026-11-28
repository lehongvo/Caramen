// ==================================================================
// PRACTICE 0994: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0994_dungeon_game.md
// Run the tests:          cargo test practice::p0994
// ==================================================================

//! # Problem 0994: Dungeon Game
//!
//! - **Difficulty:** Hard
//! - **Topics:** grid DP computed **backwards** from the goal, minimum required value, `max(1, ...)`, a padded DP table
//! - **Source:** [LeetCode 174. Dungeon Game](https://leetcode.com/problems/dungeon-game/)
//!
//! ## Problem
//!
//! A knight starts in the top-left room of an `m x n` dungeon and must reach the princess in the
//! bottom-right room, moving only **right** or **down**. Each room changes his health by
//! `dungeon[r][c]` (negative: demons, positive: orbs, zero: empty), including the first and last rooms.
//! If his health ever drops to `0` or below, he dies.
//!
//! Return the minimum **initial** health (at least 1) that lets him reach the princess.
//!
//! Implement `Solution::calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! dungeon = [[-2, -3,  3],
//!            [-5, -10, 1],
//!            [10, 30, -5]]      => 7       (path right, right, down, down)
//!
//! dungeon = [[0]]               => 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`, `-1000 <= dungeon[r][c] <= 1000`
//!
//! ## Hints
//!
//! 1. Going forwards you would need to track both the current health and the minimum so far; going
//!    **backwards** one number per room suffices.
//! 2. `need[r][c]` = minimum health when entering room `(r, c)`. In the last room,
//!    `need = max(1, 1 - dungeon[r][c])`.
//! 3. Elsewhere: `need[r][c] = max(1, min(need[r + 1][c], need[r][c + 1]) - dungeon[r][c])`.

pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tries every path (small grids only).
    fn naive(d: &[Vec<i32>]) -> i32 {
        fn walk(d: &[Vec<i32>], r: usize, c: usize, health: i32, low: i32, best: &mut i32) {
            let health = health + d[r][c];
            let low = low.min(health);
            if r + 1 == d.len() && c + 1 == d[0].len() {
                // Starting health h works if h + low >= 1.
                *best = (*best).min((1 - low).max(1));
                return;
            }
            if r + 1 < d.len() {
                walk(d, r + 1, c, health, low, best);
            }
            if c + 1 < d[0].len() {
                walk(d, r, c + 1, health, low, best);
            }
        }
        let mut best = i32::MAX;
        walk(d, 0, 0, 0, 0, &mut best);
        best
    }

    #[test]
    fn examples() {
        let d = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        assert_eq!(Solution::calculate_minimum_hp(d), 7);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
    }

    #[test]
    fn single_rooms_and_lines() {
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![-5]]), 6);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![100]]), 1);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![1, -3, 3]]), 3);
        assert_eq!(
            Solution::calculate_minimum_hp(vec![vec![0], vec![-2], vec![5], vec![-4]]),
            3
        );
    }

    #[test]
    fn orbs_later_do_not_help_earlier() {
        // The big orb comes after a deadly room: it cannot be used to survive that room.
        let d = vec![vec![-10, 1000], vec![-1000, -1]];
        assert_eq!(Solution::calculate_minimum_hp(d), 11);
    }

    #[test]
    fn matches_all_paths() {
        let d: Vec<Vec<i32>> = (0..6)
            .map(|r: i32| (0..7).map(|c: i32| ((r * 31 + c * 17) % 23) - 13).collect())
            .collect();
        assert_eq!(Solution::calculate_minimum_hp(d.clone()), naive(&d));
    }

    #[test]
    fn large_grid_of_demons() {
        let d = vec![vec![-1000; 200]; 200];
        // Any path visits 399 rooms.
        assert_eq!(Solution::calculate_minimum_hp(d), 399 * 1000 + 1);
    }
}
