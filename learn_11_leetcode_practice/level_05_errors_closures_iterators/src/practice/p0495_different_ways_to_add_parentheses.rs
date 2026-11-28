// ==================================================================
// PRACTICE 0495: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0495_different_ways_to_add_parentheses.md
// Run the tests:          cargo test practice::p0495
// ==================================================================

//! # Problem 0495: Different Ways to Add Parentheses
//!
//! - **Difficulty:** Medium
//! - **Topics:** divide and conquer with recursion, `char_indices` + `filter`, nested `flat_map` (Cartesian product), a `move` closure choosing the operator
//! - **Source:** [LeetCode 241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
//!
//! ## Problem
//!
//! `expression` contains non-negative integers and the operators `'+'`, `'-'` and `'*'` (no spaces,
//! no parentheses). Return the results of **every** way to fully parenthesise it, in any order. Two
//! different groupings that give the same value both appear in the output.
//!
//! Implement `Solution::diff_ways_to_compute(expression: String) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  expression = "2-1-1"
//! Output: [0, 2]
//! Explanation: ((2-1)-1) = 0 and (2-(1-1)) = 2.
//!
//! Input:  expression = "2*3-4*5"
//! Output: [-34, -14, -10, -10, 10]
//! Explanation: (2*(3-(4*5))) = -34, ((2*3)-(4*5)) = -14, ((2*(3-4))*5) = -10,
//!              (2*((3-4)*5)) = -10, (((2*3)-4)*5) = 10.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= expression.len() <= 20`
//! - Numbers are in `0..=99`; no leading `'-'` or `'+'`.
//! - Every intermediate and final value fits in an `i32`; there are at most `10^4` results.
//!
//! ## Hints
//!
//! 1. The **last** operator applied splits the expression into a left part and a right part. Try
//!    every operator as that last one.
//! 2. Recursively compute all results of the left part and all results of the right part, then
//!    combine every left result with every right result.
//! 3. An expression without operators is just a number.

pub struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(expr: &str) -> Vec<i32> {
        let mut out = Solution::diff_ways_to_compute(expr.to_string());
        out.sort();
        out
    }

    #[test]
    fn example_two_minus() {
        assert_eq!(run("2-1-1"), vec![0, 2]);
    }

    #[test]
    fn example_mixed_operators() {
        assert_eq!(run("2*3-4*5"), vec![-34, -14, -10, -10, 10]);
    }

    #[test]
    fn single_number_and_single_operator() {
        assert_eq!(run("42"), vec![42]);
        assert_eq!(run("0"), vec![0]);
        assert_eq!(run("99*99"), vec![9801]);
    }

    #[test]
    fn count_matches_catalan_numbers() {
        // 6 operators -> Catalan(6) = 132 groupings.
        assert_eq!(run("1+2+3+4+5+6+7").len(), 132);
        assert!(run("1+2+3+4+5+6+7").iter().all(|&v| v == 28));
        assert_eq!(run("1-1-1-1").len(), 5);
    }
}
