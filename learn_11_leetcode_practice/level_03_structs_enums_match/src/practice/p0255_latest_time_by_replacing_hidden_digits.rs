// ==================================================================
// PRACTICE 0255: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0255_latest_time_by_replacing_hidden_digits.md
// Run the tests:          cargo test practice::p0255
// ==================================================================

//! # Problem 0255: Latest Time by Replacing Hidden Digits
//!
//! - **Difficulty:** Easy
//! - **Topics:** `match` on tuples of `char`s, `char` range patterns, slice patterns on bytes, greedy choice
//! - **Source:** [LeetCode 1736. Latest Time by Replacing Hidden Digits](https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/)
//!
//! ## Problem
//!
//! You are given a time `"hh:mm"` on a 24-hour clock where some digits are hidden and shown as
//! `'?'`. Valid times go from `"00:00"` to `"23:59"`. Replace every `'?'` with a digit so that the
//! result is a **valid** time that is as **late** as possible, and return it.
//!
//! Implement `Solution::maximum_time(time: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  time = "2?:?0"
//! Output: "23:50"
//!
//! Input:  time = "0?:3?"
//! Output: "09:39"
//!
//! Input:  time = "1?:22"
//! Output: "19:22"
//!
//! Input:  time = "?4:5?"
//! Output: "14:59"   (the first digit cannot be 2, because 24 is not a valid hour)
//! ```
//!
//! ## Constraints
//!
//! - `time` has the format `"hh:mm"`; at least one valid time can be produced.
//!
//! ## Hints
//!
//! 1. The minutes are independent: a hidden tens digit becomes `'5'`, a hidden units digit `'9'`.
//! 2. The two hour digits depend on each other: match on the pair `(h1, h2)`.

pub struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn latest(t: &str) -> String {
        Solution::maximum_time(t.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(latest("2?:?0"), "23:50");
    }

    #[test]
    fn example_two() {
        assert_eq!(latest("0?:3?"), "09:39");
    }

    #[test]
    fn example_three() {
        assert_eq!(latest("1?:22"), "19:22");
    }

    #[test]
    fn first_digit_depends_on_second() {
        assert_eq!(latest("?4:5?"), "14:59");
        assert_eq!(latest("?3:00"), "23:00");
        assert_eq!(latest("?0:00"), "20:00");
    }

    #[test]
    fn all_hidden_and_none_hidden() {
        assert_eq!(latest("??:??"), "23:59");
        assert_eq!(latest("07:41"), "07:41");
        assert_eq!(latest("1?:?9"), "19:59");
    }
}
