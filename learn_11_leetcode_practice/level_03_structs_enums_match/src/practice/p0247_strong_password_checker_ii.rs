// ==================================================================
// PRACTICE 0247: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0247_strong_password_checker_ii.md
// Run the tests:          cargo test practice::p0247
// ==================================================================

//! # Problem 0247: Strong Password Checker II
//!
//! - **Difficulty:** Easy
//! - **Topics:** the `matches!` macro, `char` range patterns, `any`, `windows`, a small rules struct
//! - **Source:** [LeetCode 2299. Strong Password Checker II](https://leetcode.com/problems/strong-password-checker-ii/)
//!
//! ## Problem
//!
//! A password is **strong** when it satisfies all of these rules:
//!
//! 1. it has at least 8 characters;
//! 2. it contains at least one lowercase letter, one uppercase letter and one digit;
//! 3. it contains at least one special character from the set `!@#$%^&*()-+`;
//! 4. no two **adjacent** characters are equal (`"aab"` breaks this rule, `"aba"` does not).
//!
//! Given `password`, return `true` if it is strong.
//!
//! Implement `Solution::strong_password_checker_ii(password: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  password = "IloveLe3tcode!"
//! Output: true
//!
//! Input:  password = "Me+You--IsMyDream"
//! Output: false     (no digit, and "--" repeats a character)
//!
//! Input:  password = "1aB!"
//! Output: false     (too short)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= password.len() <= 100`
//! - `password` contains letters, digits and characters from `!@#$%^&*()-+`.
//!
//! ## Hints
//!
//! 1. `matches!(b, b'!' | b'@')` is a `bool` expression: "does `b` match this pattern?".
//!    For standard classes, `u8::is_ascii_lowercase` and friends already exist.
//! 2. `bytes.windows(2).all(|w| w[0] != w[1])` checks every adjacent pair.

pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong(p: &str) -> bool {
        Solution::strong_password_checker_ii(p.to_string())
    }

    #[test]
    fn example_strong() {
        assert!(strong("IloveLe3tcode!"));
    }

    #[test]
    fn example_missing_digit_and_repeat() {
        assert!(!strong("Me+You--IsMyDream"));
    }

    #[test]
    fn example_too_short() {
        assert!(!strong("1aB!"));
    }

    #[test]
    fn each_rule_alone_breaks_it() {
        assert!(strong("aB3!aB3!"));
        assert!(!strong("aB3!aB3")); // 7 characters
        assert!(!strong("AB3!AB3!")); // no lowercase
        assert!(!strong("ab3!ab3!")); // no uppercase
        assert!(!strong("aBc!aBc!")); // no digit
        assert!(!strong("aB3xaB3y")); // no special character
        assert!(!strong("aB3!!aB3")); // "!!" repeats
    }

    #[test]
    fn long_password() {
        let p = "aB3+".repeat(25);
        assert_eq!(p.len(), 100);
        assert!(strong(&p));
        // Same length, but every "3+a" becomes "3++": adjacent repeats.
        assert!(!strong(&p.replace("3+a", "3++")));
    }
}
