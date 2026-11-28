// ==================================================================
// PRACTICE 0222: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0222_final_value_of_variable_after_performing_operations.md
// Run the tests:          cargo test practice::p0222
// ==================================================================

//! # Problem 0222: Final Value of Variable After Performing Operations
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing strings into an `enum`, `match` on `&str` with `|` alternatives, `fold`
//! - **Source:** [LeetCode 2011. Final Value of Variable After Performing Operations](https://leetcode.com/problems/final-value-of-variable-after-performing-operations/)
//!
//! ## Problem
//!
//! A toy language has one variable `X`, which starts at `0`, and four statements:
//! `"++X"` and `"X++"` add one to `X`; `"--X"` and `"X--"` subtract one from `X`.
//!
//! You are given the program as a list of statements `operations`. Return the value of `X` after
//! running all of them in order.
//!
//! Implement `Solution::final_value_after_operations(operations: Vec<String>) -> i32`.
//!
//! For this level, first convert each statement into an `enum Op { Increment, Decrement }`, then
//! evaluate the list of `Op`s.
//!
//! ## Examples
//!
//! ```text
//! Input:  operations = ["--X", "X++", "X++"]
//! Output: 1          (0 - 1 + 1 + 1)
//!
//! Input:  operations = ["++X", "++X", "X++"]
//! Output: 3
//!
//! Input:  operations = ["X++", "++X", "--X", "X--"]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= operations.len() <= 100`
//! - Each statement is exactly one of `"++X"`, `"X++"`, `"--X"`, `"X--"`.
//!
//! ## Hints
//!
//! 1. A `match` on a `&str` can list several literals in one arm: `"++X" | "X++" => ...`.
//! 2. `operations` holds `String`s; `s.as_str()` gives the `&str` to match on.

pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ops(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["--X", "X++", "X++"])),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["++X", "++X", "X++"])),
            3
        );
    }

    #[test]
    fn example_three_cancels_out() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["X++", "++X", "--X", "X--"])),
            0
        );
    }

    #[test]
    fn single_decrement() {
        assert_eq!(Solution::final_value_after_operations(ops(&["X--"])), -1);
    }

    #[test]
    fn hundred_decrements() {
        let program = vec!["--X".to_string(); 100];
        assert_eq!(Solution::final_value_after_operations(program), -100);
    }
}
