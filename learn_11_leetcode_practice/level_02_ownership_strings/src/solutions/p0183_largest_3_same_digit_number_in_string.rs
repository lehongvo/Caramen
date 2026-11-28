//! # Problem 0183: Largest 3-Same-Digit Number in String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `as_bytes().windows(3)`, slice patterns on windows, `max`, building a `String` from a byte
//! - **Source:** [LeetCode 2264. Largest 3-Same-Digit Number in String](https://leetcode.com/problems/largest-3-same-digit-number-in-string/)
//!
//! ## Problem
//!
//! A substring of `num` is *good* if it has length 3 and consists of one digit repeated three times
//! (such as `"777"` or `"000"`). Return the good substring with the largest value, or an empty string
//! if `num` has no good substring.
//!
//! Implement `Solution::largest_good_integer(num: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = "6777133339"
//! Output: "777"            ("777" and "333" are good; 777 is larger)
//!
//! Input:  num = "2300019"
//! Output: "000"
//!
//! Input:  num = "42352338"
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `3 <= num.len() <= 1000`
//! - `num` consists only of digits.
//!
//! ## Hints
//!
//! 1. `num.as_bytes().windows(3)` yields every substring of length 3 as a `&[u8]`.
//! 2. Comparing good substrings is the same as comparing their digit, so keep the maximum digit.
//! 3. Rebuild the answer with `(digit as char).to_string().repeat(3)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** scan all windows of three bytes, keep the digit of every window whose three bytes are
//! equal, and take the maximum. ASCII digits compare in numeric order, so the largest byte is the
//! largest good number.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - A slice pattern in a closure, `|w| matches!(w, [a, b, c] if a == b && b == c)`, reads the three
//!   elements of a window by name.
//! - `Iterator::max` returns `Option<u8>`; `map_or(String::new(), ..)` handles "none found".
//! - `u8 as char` converts an ASCII byte to the matching character.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        // @solution-start
        num.as_bytes()
            .windows(3)
            .filter(|w| matches!(w, [a, b, c] if a == b && b == c))
            .map(|w| w[0])
            .max()
            .map_or(String::new(), |digit| (digit as char).to_string().repeat(3))
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(num: &str) -> String {
        Solution::largest_good_integer(num.to_string())
    }

    #[test]
    fn example_seven() {
        assert_eq!(run("6777133339"), "777");
    }

    #[test]
    fn example_zeros() {
        assert_eq!(run("2300019"), "000");
    }

    #[test]
    fn example_none() {
        assert_eq!(run("42352338"), "");
    }

    #[test]
    fn whole_string_and_longer_runs() {
        assert_eq!(run("999"), "999");
        assert_eq!(run("11112222"), "222");
    }

    #[test]
    fn largest_input() {
        let num = format!("{}888{}", "12".repeat(400), "3".repeat(197));
        assert_eq!(run(&num), "888");
    }
}
