// ==================================================================
// PRACTICE 0469: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0469_reformat_the_string.md
// Run the tests:          cargo test practice::p0469
// ==================================================================

//! # Problem 0469: Reformat The String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `partition` on `chars`, `zip` with `chain(repeat(None))`, `flat_map` to interleave, `iter::once`
//! - **Source:** [LeetCode 1417. Reformat The String](https://leetcode.com/problems/reformat-the-string/)
//!
//! ## Problem
//!
//! `s` contains lowercase English letters and digits. Rearrange its characters so that no two
//! letters are next to each other and no two digits are next to each other (letters and digits
//! alternate). Return **any** such arrangement, or the empty string if it is impossible.
//!
//! Implement `Solution::reformat(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "a0b1c2"
//! Output: "0a1b2c"          (also accepted: "a0b1c2", "0c2a1b", ...)
//!
//! Input:  s = "leetcode"
//! Output: ""                (only letters: impossible)
//!
//! Input:  s = "1229857369"
//! Output: ""                (only digits: impossible)
//!
//! Input:  s = "covid2019"
//! Output: "c2o0v1i9d"       (5 letters and 4 digits: the letters must come first and last)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 500`
//! - `s` contains only lowercase English letters and/or digits.
//!
//! ## Hints
//!
//! 1. `s.chars().partition(|c| c.is_ascii_digit())` splits the characters into digits and letters.
//! 2. It is possible exactly when the two counts differ by at most 1; the larger group must start.
//! 3. To interleave, pair each character of the larger group with an `Option` from the smaller one:
//!    `small.iter().map(Some).chain(iter::repeat(None))` never runs out.

use std::iter;

pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that `out` is a valid rearrangement of `input` (or that it is empty when impossible).
    fn check(input: &str, possible: bool) {
        let out = Solution::reformat(input.to_string());
        if !possible {
            assert_eq!(out, "");
            return;
        }
        let mut a: Vec<char> = input.chars().collect();
        let mut b: Vec<char> = out.chars().collect();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b, "the output must use exactly the input characters");
        let kinds: Vec<bool> = out.chars().map(|c| c.is_ascii_digit()).collect();
        assert!(
            kinds.windows(2).all(|w| w[0] != w[1]),
            "`{out}` does not alternate"
        );
    }

    #[test]
    fn example_balanced() {
        check("a0b1c2", true);
    }

    #[test]
    fn examples_impossible() {
        check("leetcode", false);
        check("1229857369", false);
        check("ab1", true);
        check("abc1", false);
    }

    #[test]
    fn example_one_more_letter() {
        check("covid2019", true);
        assert_eq!(Solution::reformat("covid2019".to_string()), "c2o0v1i9d");
    }

    #[test]
    fn single_characters_and_long_input() {
        check("a", true);
        check("7", true);
        let long: String = (0..250)
            .map(|i| if i % 2 == 0 { 'x' } else { '5' })
            .chain("y5".chars())
            .collect();
        check(&long, true);
    }
}
