// ==================================================================
// PRACTICE 0018: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0018_nim_game.md
// Run the tests:          cargo test practice::p0018
// ==================================================================

//! # Problem 0018: Nim Game
//!
//! - **Difficulty:** Easy
//! - **Topics:** `%`, `bool` results, finding a pattern
//! - **Source:** [LeetCode 292. Nim Game](https://leetcode.com/problems/nim-game/)
//!
//! ## Problem
//!
//! You and a friend play a game with a single pile of `n` stones. You move first, and then you
//! alternate. On each move a player must take 1, 2 or 3 stones from the pile. The player who takes
//! the **last** stone wins.
//!
//! Both players play perfectly. Return `true` if you can force a win, and `false` otherwise.
//!
//! Implement `Solution::can_win_nim(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4
//! Output: false
//! Explanation: whatever you take (1, 2 or 3), your friend takes the rest and wins.
//!
//! Input:  n = 1
//! Output: true
//!
//! Input:  n = 7
//! Output: true
//! Explanation: take 3, leaving 4 stones; now your friend is in the losing position above.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Write down who wins for `n = 1, 2, 3, ..., 9`. Which values lose?
//! 2. `n` can be about two billion, so a loop or recursion over `n` is far too slow.

pub struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_loses() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn examples_that_win() {
        assert!(Solution::can_win_nim(1));
        assert!(Solution::can_win_nim(2));
        assert!(Solution::can_win_nim(7));
    }

    #[test]
    fn matches_brute_force() {
        // wins[k] is true when the player to move with k stones can win.
        let mut wins = [false; 101];
        for k in 1..=100 {
            wins[k] = (1..=3).any(|take| take <= k && !wins[k - take]);
        }
        for (k, &win) in wins.iter().enumerate().skip(1) {
            assert_eq!(Solution::can_win_nim(k as i32), win, "n = {k}");
        }
    }

    #[test]
    fn largest_input() {
        // 2^31 - 1 = 2147483647 leaves remainder 3 when divided by 4.
        assert!(Solution::can_win_nim(i32::MAX));
        assert!(!Solution::can_win_nim(2_147_483_644));
    }
}
