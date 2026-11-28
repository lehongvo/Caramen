// ==================================================================
// PRACTICE 0069: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0069_distribute_candies_among_children_i.md
// Run the tests:          cargo test practice::p0069
// ==================================================================

//! # Problem 0069: Distribute Candies Among Children I
//!
//! - **Difficulty:** Easy
//! - **Topics:** nested `for` loops, deriving the last variable instead of looping over it, ranges
//! - **Source:** [LeetCode 2928. Distribute Candies Among Children I](https://leetcode.com/problems/distribute-candies-among-children-i/)
//!
//! ## Problem
//!
//! You must give **all** `n` candies to 3 children so that no child receives more than `limit`
//! candies (a child may receive 0). Two distributions are different if at least one child gets a
//! different number of candies. Return the number of possible distributions.
//!
//! Implement `Solution::distribute_candies(n: i32, limit: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 5, limit = 2
//! Output: 3
//! Explanation: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
//!
//! Input:  n = 3, limit = 3
//! Output: 10
//! Explanation: (0,0,3) (0,1,2) (0,2,1) (0,3,0) (1,0,2) (1,1,1) (1,2,0) (2,0,1) (2,1,0) (3,0,0).
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 50`
//! - `1 <= limit <= 50`
//!
//! ## Hints
//!
//! 1. Try every amount `a` for the first child and `b` for the second child, both in `0..=limit`.
//! 2. The third child's amount is then forced: `c = n - a - b`. Check that `0 <= c <= limit`.

pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_two() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
    }

    #[test]
    fn example_three_three() {
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }

    #[test]
    fn impossible_when_limit_too_small() {
        assert_eq!(Solution::distribute_candies(7, 2), 0);
        assert_eq!(Solution::distribute_candies(6, 2), 1);
    }

    #[test]
    fn matches_triple_loop() {
        for n in 1..=20 {
            for limit in 1..=20 {
                let mut expected = 0;
                for a in 0..=limit {
                    for b in 0..=limit {
                        for c in 0..=limit {
                            if a + b + c == n {
                                expected += 1;
                            }
                        }
                    }
                }
                assert_eq!(Solution::distribute_candies(n, limit), expected);
            }
        }
    }

    #[test]
    fn largest_inputs() {
        // With limit >= n, every split counts: (n + 2) choose 2 = 52 * 51 / 2.
        assert_eq!(Solution::distribute_candies(50, 50), 1326);
    }
}
