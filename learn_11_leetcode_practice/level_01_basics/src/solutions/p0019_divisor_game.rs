//! # Problem 0019: Divisor Game
//!
//! - **Difficulty:** Easy
//! - **Topics:** parity, `bool` results, reasoning about games
//! - **Source:** [LeetCode 1025. Divisor Game](https://leetcode.com/problems/divisor-game/)
//!
//! ## Problem
//!
//! Alice and Bob play a game with a number `n` written on a board. Alice moves first and then
//! they alternate. On a move, the current player must:
//!
//! 1. pick an integer `x` with `0 < x < n` that divides `n` exactly (`n % x == 0`), and
//! 2. replace `n` on the board with `n - x`.
//!
//! A player who cannot make a move loses. Both players play perfectly. Return `true` if Alice
//! wins and `false` otherwise.
//!
//! Implement `Solution::divisor_game(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2
//! Output: true
//! Explanation: Alice picks 1, the board shows 1, and Bob has no legal move.
//!
//! Input:  n = 3
//! Output: false
//! Explanation: Alice must pick 1 (the only divisor below 3). Bob gets 2, picks 1, and Alice is stuck.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//!
//! ## Hints
//!
//! 1. Work out the winner for `n = 1, 2, 3, 4, 5, 6`. Do you see a pattern?
//! 2. Every divisor of an odd number is odd. What is the parity of `n - x` then?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** Alice wins exactly when `n` is even. From an even `n`, she can always pick `x = 1`
//! and hand Bob an odd number. From an odd `n`, every divisor `x` is odd, so `n - x` is even: the
//! opponent always receives an even number. Eventually the player with the odd number faces `1`
//! and loses.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `n % 2 == 0` is the idiomatic even test for signed and unsigned integers.
//! - The tests verify the formula with a brute-force table built with a `bool` array and nested loops.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        // @solution-start
        n % 2 == 0
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two() {
        assert!(Solution::divisor_game(2));
    }

    #[test]
    fn example_three() {
        assert!(!Solution::divisor_game(3));
    }

    #[test]
    fn one_is_a_loss() {
        assert!(!Solution::divisor_game(1));
    }

    #[test]
    fn matches_brute_force_up_to_1000() {
        // wins[k] is true when the player to move with k on the board can win.
        let mut wins = [false; 1001];
        for k in 2..=1000 {
            wins[k] = (1..k).any(|x| k % x == 0 && !wins[k - x]);
        }
        for (k, &win) in wins.iter().enumerate().skip(1) {
            assert_eq!(Solution::divisor_game(k as i32), win, "n = {k}");
        }
    }
}
