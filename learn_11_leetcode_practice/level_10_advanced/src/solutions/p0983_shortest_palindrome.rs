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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** compute the prefix function `pi` of `t = s # reverse(s)`: `pi[i]` is the length of the
//! longest proper prefix of `t[..=i]` that is also its suffix. At the last position this is the
//! longest prefix of `s` equal to a suffix of `reverse(s)`, i.e. the longest palindromic prefix.
//! Prepend the reverse of the remaining suffix.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - Working on `Vec<u8>` is safe here because the input is ASCII; `String::from_utf8` would also
//!   accept the result.
//! - The prefix-function loop `while k > 0 && t[i] != t[k] { k = pi[k - 1]; }` is the heart of KMP.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // @solution-start
        let reversed: String = s.chars().rev().collect();
        let t: Vec<u8> = s
            .bytes()
            .chain(std::iter::once(b'#'))
            .chain(reversed.bytes())
            .collect();
        let mut pi = vec![0_usize; t.len()];
        for i in 1..t.len() {
            let mut k = pi[i - 1];
            while k > 0 && t[i] != t[k] {
                k = pi[k - 1];
            }
            if t[i] == t[k] {
                k += 1;
            }
            pi[i] = k;
        }
        let palindromic_prefix = pi[t.len() - 1];
        format!("{}{}", &reversed[..s.len() - palindromic_prefix], s)
        // @solution-end
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
