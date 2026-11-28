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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** parse, then replay the operations on a `Vec<i32>` used as a stack; the answer is the
//! sum of the stack.
//!
//! **Complexity:** `O(n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - The last arm of a `match` on `&str` can be a binding (`number => ...`) that catches every other
//!   string, like `_` but with a name.
//! - `Op::Score` is a function `fn(i32) -> Op`, so `.map(Op::Score)` wraps a parsed number.
//! - Slice patterns on the stack (`if let [.., a, b] = scores[..]`) read the last two scores
//!   without index arithmetic; on invalid input the operation is simply skipped.
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Score(i32),
    Sum,
    Double,
    Cancel,
}

impl Op {
    fn parse(s: &str) -> Option<Op> {
        match s {
            "+" => Some(Op::Sum),
            "D" => Some(Op::Double),
            "C" => Some(Op::Cancel),
            number => number.parse().ok().map(Op::Score),
        }
    }
}
// @solution-end

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        // @solution-start
        let mut scores: Vec<i32> = Vec::with_capacity(operations.len());
        for op in operations.iter().filter_map(|s| Op::parse(s)) {
            match op {
                Op::Score(x) => scores.push(x),
                Op::Sum => {
                    if let [.., a, b] = scores[..] {
                        scores.push(a + b);
                    }
                }
                Op::Double => {
                    if let Some(&last) = scores.last() {
                        scores.push(2 * last);
                    }
                }
                Op::Cancel => {
                    scores.pop();
                }
            }
        }
        scores.iter().sum()
        // @solution-end
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
