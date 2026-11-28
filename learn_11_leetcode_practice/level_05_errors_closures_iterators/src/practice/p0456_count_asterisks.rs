// ==================================================================
// PRACTICE 0456: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0456_count_asterisks.md
// Run the tests:          cargo test practice::p0456
// ==================================================================

//! # Problem 0456: Count Asterisks
//!
//! - **Difficulty:** Easy
//! - **Topics:** `str::split`, `Iterator::step_by`, `str::matches`, `map` + `sum`
//! - **Source:** [LeetCode 2315. Count Asterisks](https://leetcode.com/problems/count-asterisks/)
//!
//! ## Problem
//!
//! The string `s` contains bars `'|'` that come in pairs: the 1st and 2nd bar form a pair, the 3rd
//! and 4th form the next pair, and so on. Count the asterisks `'*'` in `s` that are **not** between
//! the two bars of a pair.
//!
//! Implement `Solution::count_asterisks(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "l|*e*et|c**o|*de|"
//! Output: 2
//! Explanation: "*e*et" and "*de" are inside pairs; only the two asterisks of "c**o" count.
//!
//! Input:  s = "iamprogrammer"
//! Output: 0
//!
//! Input:  s = "yo|uar|e**|b|e***au|tifu|l"
//! Output: 5
//! Explanation: the parts outside pairs are "yo", "e**", "e***au" and "l".
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 1000`
//! - `s` contains lowercase English letters, `'|'` and `'*'`.
//! - `s` contains an **even** number of `'|'`.
//!
//! ## Hints
//!
//! 1. Splitting on `'|'` gives the parts between bars. Which parts are outside the pairs: the ones
//!    with an even index or an odd index?
//! 2. `step_by(2)` keeps every second item of an iterator, starting with the first.

pub struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> i32 {
        Solution::count_asterisks(s.to_string())
    }

    #[test]
    fn example_mixed() {
        assert_eq!(run("l|*e*et|c**o|*de|"), 2);
    }

    #[test]
    fn example_no_bars() {
        assert_eq!(run("iamprogrammer"), 0);
    }

    #[test]
    fn example_many_pairs() {
        assert_eq!(run("yo|uar|e**|b|e***au|tifu|l"), 5);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(run("*"), 1);
        assert_eq!(run("|*|"), 0);
        assert_eq!(run("||**||"), 2);
        assert_eq!(run(&"*".repeat(1000)), 1000);
    }
}
