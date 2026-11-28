// ==================================================================
// PRACTICE 0983: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0983_shortest_palindrome.md
// Run the tests:          cargo test practice::p0983
// ==================================================================

//! # Problem 0983: Shortest Palindrome
//!
//! - **Difficulty:** Hard
//! - **Topics:** KMP prefix function (failure function), longest palindromic prefix, building a combined byte string with a separator
//! - **Source:** [LeetCode 214. Shortest Palindrome](https://leetcode.com/problems/shortest-palindrome/)
//!
//! ## Problem
//!
//! You may only add characters **in front of** the string `s`. Return the shortest palindrome you can
//! obtain this way.
//!
//! Implement `Solution::shortest_palindrome(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aacecaaa"      Output: "aaacecaaa"
//! Input:  s = "abcd"          Output: "dcbabcd"
//! Input:  s = ""              Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 5 * 10^4`
//! - `s` consists of lowercase English letters.
//!
//! ## Hints
//!
//! 1. The answer is `reverse(rest) + s`, where `s = prefix + rest` and `prefix` is the **longest
//!    palindromic prefix** of `s`.
//! 2. The longest palindromic prefix is the longest prefix of `s` that is also a suffix of
//!    `reverse(s)`.
//! 3. The KMP prefix function of `s + "#" + reverse(s)` gives exactly that length at its last position
//!    (the `#` stops matches from crossing the middle).

pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::shortest_palindrome(s.to_string())
    }

    fn is_palindrome(s: &str) -> bool {
        s.bytes().eq(s.bytes().rev())
    }

    fn naive(s: &str) -> String {
        let longest = (0..=s.len())
            .rev()
            .find(|&k| is_palindrome(&s[..k]))
            .unwrap_or(0);
        let rest: String = s[longest..].chars().rev().collect();
        rest + s
    }

    #[test]
    fn examples() {
        assert_eq!(run("aacecaaa"), "aaacecaaa");
        assert_eq!(run("abcd"), "dcbabcd");
        assert_eq!(run(""), "");
    }

    #[test]
    fn already_palindromes() {
        assert_eq!(run("a"), "a");
        assert_eq!(run("racecar"), "racecar");
        assert_eq!(run("aa"), "aa");
        assert_eq!(run("ab"), "bab");
    }

    #[test]
    fn matches_naive() {
        for mask in 0..(1_u32 << 10) {
            let s: String = (0..10)
                .map(|b| if (mask >> b) & 1 == 1 { 'a' } else { 'b' })
                .collect();
            assert_eq!(run(&s), naive(&s), "{s}");
        }
    }

    #[test]
    fn long_input() {
        // The longest palindromic prefix is a^24999 b a^24999: only one `a` must be added.
        let s = "a".repeat(24_999) + "b" + &"a".repeat(25_000);
        let out = run(&s);
        assert_eq!(out.len(), 50_001);
        assert!(is_palindrome(&out));
        assert!(out.ends_with(&s));
    }
}
