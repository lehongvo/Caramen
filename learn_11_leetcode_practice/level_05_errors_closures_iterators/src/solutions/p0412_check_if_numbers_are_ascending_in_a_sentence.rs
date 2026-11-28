//! # Problem 0412: Check if Numbers Are Ascending in a Sentence
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split`, `filter_map`, `Result::ok`, `windows`, `Iterator::all`
//! - **Source:** [LeetCode 2042. Check if Numbers Are Ascending in a Sentence](https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/)
//!
//! ## Problem
//!
//! A sentence is a list of *tokens* separated by single spaces. Each token is either a positive
//! number written with digits only (no leading zeros) or a word made of lowercase English letters.
//!
//! Return `true` if all the numbers in the sentence are **strictly increasing** from left to right
//! (each number is smaller than the next number), and `false` otherwise. The words are ignored.
//!
//! Implement `Solution::are_numbers_ascending(s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
//! Output: true
//! Explanation: the numbers are 1, 3, 4, 6, 12.
//!
//! Input:  s = "hello world 5 x 5"
//! Output: false
//! Explanation: 5 is not strictly smaller than 5.
//!
//! Input:  s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
//! Output: false
//! Explanation: 51 is followed by 50.
//! ```
//!
//! ## Constraints
//!
//! - `3 <= s.len() <= 200`
//! - `s` has lowercase English letters, digits and single spaces; no leading or trailing spaces.
//! - Numbers are positive, below `100`, without leading zeros. There are at least two numbers.
//!
//! ## Hints
//!
//! 1. A token is a number exactly when `token.parse::<u32>()` succeeds.
//! 2. `filter_map(|t| t.parse::<u32>().ok())` keeps only the numbers, already converted.
//! 3. Compare neighbours with `windows(2)` on a `Vec`, or remember the previous number while
//!    iterating.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** turn the sentence into the list of its numbers with one `filter_map`: parsing a word
//! fails, `.ok()` turns that failure into `None`, and `filter_map` drops it. Then check that every
//! pair of neighbours is strictly increasing.
//!
//! **Complexity:** `O(n)` time, `O(k)` space for the `k` numbers.
//!
//! **Rust concepts:**
//! - `filter_map` combines `map` and `filter`: the closure returns an `Option`, `None` items are
//!   skipped and `Some(x)` items are unwrapped.
//! - `Result::ok()` converts `Result<T, E>` into `Option<T>`, which is what `filter_map` expects.
//! - `slice::windows(2)` yields overlapping sub-slices `[a, b]`; `all` stops at the first `false`.
//! - Without the intermediate `Vec`, you could write
//!   `.try_fold(0, |prev, n| (n > prev).then_some(n)).is_some()`, since all numbers are positive.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        // @solution-start
        let numbers: Vec<u32> = s.split(' ').filter_map(|t| t.parse().ok()).collect();
        numbers.windows(2).all(|w| w[0] < w[1])
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> bool {
        Solution::are_numbers_ascending(s.to_string())
    }

    #[test]
    fn example_marbles() {
        assert!(run("1 box has 3 blue 4 red 6 green and 12 yellow marbles"));
    }

    #[test]
    fn example_equal_numbers() {
        assert!(!run("hello world 5 x 5"));
    }

    #[test]
    fn example_decrease_later() {
        assert!(!run(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
        ));
    }

    #[test]
    fn only_numbers() {
        assert!(run("1 2 3 99"));
        assert!(!run("2 1"));
        assert!(run("4 5"));
    }
}
