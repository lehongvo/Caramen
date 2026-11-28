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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** two nested loops choose `a` and `b`; `c = n - a - b` is determined, and the
//! distribution is valid when `c` lies in `0..=limit`. This avoids a third loop.
//!
//! **Complexity:** `O(limit^2)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Nested `for` loops over inclusive ranges `0..=limit`.
//! - `(0..=limit).contains(&c)` is the idiomatic range check (Clippy prefers it over
//!   `c >= 0 && c <= limit`). `contains` takes a reference, hence `&c`.
//! - `c` may be negative, which is why the computation stays in `i32` rather than `usize`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        // @solution-start
        let mut ways = 0;
        for a in 0..=limit {
            for b in 0..=limit {
                let c = n - a - b;
                if (0..=limit).contains(&c) {
                    ways += 1;
                }
            }
        }
        ways
        // @solution-end
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
