// ==================================================================
// PRACTICE 0992: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0992_palindrome_partitioning_ii.md
// Run the tests:          cargo test practice::p0992
// ==================================================================

//! # Problem 0992: Palindrome Partitioning II
//!
//! - **Difficulty:** Hard
//! - **Topics:** DP over prefixes combined with expand-around-center palindrome enumeration, `O(n)` memory
//! - **Source:** [LeetCode 132. Palindrome Partitioning II](https://leetcode.com/problems/palindrome-partitioning-ii/)
//!
//! ## Problem
//!
//! Split the string `s` into pieces that are all palindromes, using as few **cuts** as possible, and
//! return that minimum number of cuts.
//!
//! Implement `Solution::min_cut(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! s = "aab"   => 1     ("aa" | "b")
//! s = "a"     => 0
//! s = "ab"    => 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 2000`, lowercase English letters.
//!
//! ## Hints
//!
//! 1. Let `cuts[i]` be the minimum number of cuts for the prefix `s[..i]`, with `cuts[0] = -1` (so a
//!    prefix that is a palindrome costs `-1 + 1 = 0`).
//! 2. Every palindrome `s[l..r]` gives `cuts[r] = min(cuts[r], cuts[l] + 1)`.
//! 3. Enumerate all palindromes by expanding around each center (odd and even lengths): `O(n^2)` in
//!    total, without an `n x n` table.

pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> i32 {
        Solution::min_cut(s.to_string())
    }

    fn naive(s: &[u8]) -> i32 {
        let n = s.len();
        let is_pal = |l: usize, r: usize| s[l..r].iter().eq(s[l..r].iter().rev());
        let mut best = vec![i32::MAX; n + 1];
        best[0] = -1;
        for r in 1..=n {
            for l in 0..r {
                if is_pal(l, r) {
                    best[r] = best[r].min(best[l] + 1);
                }
            }
        }
        best[n]
    }

    #[test]
    fn examples() {
        assert_eq!(run("aab"), 1);
        assert_eq!(run("a"), 0);
        assert_eq!(run("ab"), 1);
    }

    #[test]
    fn palindromes_need_no_cut() {
        assert_eq!(run("racecar"), 0);
        assert_eq!(run("abba"), 0);
        assert_eq!(run("abcba"), 0);
        assert_eq!(run("abcd"), 3);
        assert_eq!(run("cdd"), 1);
    }

    #[test]
    fn matches_naive() {
        for mask in 0..(1_u32 << 11) {
            let s: Vec<u8> = (0..11)
                .map(|bit| if (mask >> bit) & 1 == 1 { b'a' } else { b'b' })
                .collect();
            let text = String::from_utf8(s.clone()).unwrap();
            assert_eq!(run(&text), naive(&s), "{text}");
        }
    }

    #[test]
    fn long_inputs() {
        assert_eq!(run(&"a".repeat(2_000)), 0);
        let s: String = "ab".repeat(1_000);
        assert_eq!(run(&s), 1);
        let distinct: String = (0..2_000)
            .map(|i| char::from(b'a' + (i % 26) as u8))
            .collect();
        assert_eq!(run(&distinct), 1_999);
    }
}
