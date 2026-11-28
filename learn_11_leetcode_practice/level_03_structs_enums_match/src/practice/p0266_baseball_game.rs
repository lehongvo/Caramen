// ==================================================================
// PRACTICE 0266: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0266_baseball_game.md
// Run the tests:          cargo test practice::p0266
// ==================================================================

//! # Problem 0266: Baseball Game
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing strings into an operation `enum` with data, `match` on `&str` with a binding fallback, slice patterns on a stack
//! - **Source:** [LeetCode 682. Baseball Game](https://leetcode.com/problems/baseball-game/)
//!
//! ## Problem
//!
//! You keep the score sheet of a game with unusual rules. The sheet starts empty and you process
//! the list `operations`, where each operation is one of:
//!
//! - an integer `x` (as a string, possibly negative): record a new score `x`;
//! - `"+"`: record a new score equal to the **sum of the previous two** scores;
//! - `"D"`: record a new score equal to **double the previous** score;
//! - `"C"`: **remove** the previous score from the sheet.
//!
//! The operations are guaranteed to be valid (there are always enough previous scores). Return the
//! sum of all scores on the sheet at the end.
//!
//! Implement `Solution::cal_points(operations: Vec<String>) -> i32`.
//!
//! For this level, parse each string into `enum Op { Score(i32), Sum, Double, Cancel }` first.
//!
//! ## Examples
//!
//! ```text
//! Input:  operations = ["5", "2", "C", "D", "+"]
//! Output: 30     (sheet: [5] -> [5, 2] -> [5] -> [5, 10] -> [5, 10, 15])
//!
//! Input:  operations = ["5", "-2", "4", "C", "D", "9", "+", "+"]
//! Output: 27     (final sheet: [5, -2, -4, 9, 5, 14])
//!
//! Input:  operations = ["1", "C"]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= operations.len() <= 1000`
//! - Scores are in `[-3 * 10^4, 3 * 10^4]`; all intermediate values fit in an `i32`.
//!
//! ## Hints
//!
//! 1. `match s { "+" => Some(Op::Sum), "D" => ..., "C" => ..., number => number.parse().ok().map(Op::Score) }`.
//! 2. On the stack of scores, `[.., a, b]` names the last two elements.

pub struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
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
        assert_eq!(Solution::cal_points(ops(&["5", "2", "C", "D", "+"])), 30);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::cal_points(ops(&["5", "-2", "4", "C", "D", "9", "+", "+"])),
            27
        );
    }

    #[test]
    fn example_cancel_everything() {
        assert_eq!(Solution::cal_points(ops(&["1", "C"])), 0);
    }

    #[test]
    fn negative_doubles_and_sums() {
        assert_eq!(Solution::cal_points(ops(&["-3", "D", "+"])), -3 - 6 - 9);
    }

    #[test]
    fn many_operations() {
        // [1] -> [1, 2] -> [1, 2, 4] -> [1, 2, 4, 8] -> cancel -> then 995 sevens.
        let mut list = vec!["1", "D", "D", "D", "C"];
        list.extend(vec!["7"; 995]);
        assert_eq!(list.len(), 1000);
        assert_eq!(Solution::cal_points(ops(&list)), 1 + 2 + 4 + 7 * 995);
    }
}
