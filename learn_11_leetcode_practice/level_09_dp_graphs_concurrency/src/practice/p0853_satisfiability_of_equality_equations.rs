// ==================================================================
// PRACTICE 0853: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0853_satisfiability_of_equality_equations.md
// Run the tests:          cargo test practice::p0853
// ==================================================================

//! # Problem 0853: Satisfiability of Equality Equations
//!
//! - **Difficulty:** Medium
//! - **Topics:** union-find over a fixed alphabet (`[usize; 26]`), two passes, byte patterns
//! - **Source:** [LeetCode 990. Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations/)
//!
//! ## Problem
//!
//! Each string in `equations` has length 4 and is either `"x==y"` or `"x!=y"`, where `x` and `y`
//! are lowercase letters naming integer variables (they may be the same letter). Return `true` if
//! integers can be assigned to the variables so that every equation holds at the same time.
//!
//! Implement `Solution::equations_possible(equations: Vec<String>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  equations = ["a==b", "b!=a"]
//! Output: false
//!
//! Input:  equations = ["b==a", "a==b"]
//! Output: true
//!
//! Input:  equations = ["a==b", "b!=c", "c==a"]
//! Output: false
//! Explanation: a == b and c == a force b == c.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= equations.len() <= 500`
//! - `equations[i].len() == 4`; characters 0 and 3 are lowercase letters; characters 1-2 are
//!   `"=="` or `"!="`.
//!
//! ## Hints
//!
//! 1. Equalities group the variables into classes (equality is transitive).
//! 2. Process all `==` equations first, then check that no `!=` equation has both sides in the same
//!    class. The order of the equations in the input does not matter.

pub struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(eqs: &[&str]) -> bool {
        Solution::equations_possible(eqs.iter().map(|e| e.to_string()).collect())
    }

    #[test]
    fn example_contradiction() {
        assert!(!run(&["a==b", "b!=a"]));
    }

    #[test]
    fn example_consistent() {
        assert!(run(&["b==a", "a==b"]));
    }

    #[test]
    fn example_transitivity() {
        assert!(!run(&["a==b", "b!=c", "c==a"]));
    }

    #[test]
    fn same_letter() {
        assert!(!run(&["a!=a"]));
        assert!(run(&["c==c", "b==d", "x!=z"]));
    }

    #[test]
    fn inequality_before_equalities() {
        // The order of the input does not matter: z != a is checked after all equalities.
        let mut eqs = vec!["z!=a".to_string()];
        eqs.extend((b'a'..=b'y').map(|c| format!("{}=={}", c as char, (c + 1) as char)));
        assert!(!Solution::equations_possible(eqs));
    }
}
