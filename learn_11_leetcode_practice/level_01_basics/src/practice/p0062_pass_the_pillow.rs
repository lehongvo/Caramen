// ==================================================================
// PRACTICE 0062: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0062_pass_the_pillow.md
// Run the tests:          cargo test practice::p0062
// ==================================================================

//! # Problem 0062: Pass the Pillow
//!
//! - **Difficulty:** Easy
//! - **Topics:** `%` on a cycle, `if` as an expression, checking a formula against a simulation
//! - **Source:** [LeetCode 2582. Pass the Pillow](https://leetcode.com/problems/pass-the-pillow/)
//!
//! ## Problem
//!
//! `n` people stand in a line, numbered `1` to `n`. Person `1` holds a pillow. Every second the
//! holder passes it to the next person in the current direction. When the pillow reaches either
//! end of the line, the direction reverses: after reaching person `n` it goes back to `n - 1`,
//! `n - 2`, ..., and after reaching person `1` it goes forward again.
//!
//! Return the number of the person holding the pillow after `time` seconds.
//!
//! Implement `Solution::pass_the_pillow(n: i32, time: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, time = 5
//! Output: 2
//! Explanation: 1 -> 2 -> 3 -> 4 -> 3 -> 2.
//!
//! Input:  n = 3, time = 2
//! Output: 3
//! Explanation: 1 -> 2 -> 3.
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 1000`
//! - `1 <= time <= 1000`
//!
//! ## Hints
//!
//! 1. A full trip forward and back takes `2 * (n - 1)` seconds and returns the pillow to person 1.
//! 2. Reduce `time` modulo that period, then decide whether the pillow is on its way forward or back.

pub struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulate(n: i32, time: i32) -> i32 {
        let mut person = 1;
        let mut direction = 1;
        for _ in 0..time {
            if person + direction < 1 || person + direction > n {
                direction = -direction;
            }
            person += direction;
        }
        person
    }

    #[test]
    fn example_four_people() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
    }

    #[test]
    fn example_three_people() {
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
    }

    #[test]
    fn two_people_alternate() {
        assert_eq!(Solution::pass_the_pillow(2, 1), 2);
        assert_eq!(Solution::pass_the_pillow(2, 2), 1);
        assert_eq!(Solution::pass_the_pillow(2, 1000), 1);
    }

    #[test]
    fn matches_simulation() {
        for n in 2..=30 {
            for time in 1..=200 {
                assert_eq!(
                    Solution::pass_the_pillow(n, time),
                    simulate(n, time),
                    "n={n}, time={time}"
                );
            }
        }
        assert_eq!(Solution::pass_the_pillow(1000, 1000), simulate(1000, 1000));
    }
}
