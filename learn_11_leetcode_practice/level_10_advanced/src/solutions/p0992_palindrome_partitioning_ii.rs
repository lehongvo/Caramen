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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** expand around every center, left to right; each palindrome `s[l..=r]` found relaxes
//! `cuts[r + 1]` with `cuts[l] + 1`. When the center is `c`, the value `cuts[l]` (with `l <= c`) is
//! already final: it only depends on palindromes ending at `l - 1`, whose centers are all left of
//! `c` and were processed earlier.
//!
//! **Complexity:** `O(n^2)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `i32` cuts with a `-1` sentinel keep the recurrence uniform.
//! - A small closure `expand(l, r)` handles both odd (`l == r`) and even (`l + 1 == r`) centers.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        // @solution-start
        let b = s.as_bytes();
        let n = b.len();
        // cuts[i] = minimum cuts for s[..i]; start from the worst case (cut between every letter).
        let mut cuts: Vec<i32> = (0..=n as i32).map(|i| i - 1).collect();
        for center in 0..n {
            for (mut l, mut r) in [(center, center), (center, center + 1)] {
                // Invariant: s[l..=r] is a palindrome while the loop continues.
                while r < n && b[l] == b[r] {
                    cuts[r + 1] = cuts[r + 1].min(cuts[l] + 1);
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    r += 1;
                }
            }
        }
        cuts[n]
        // @solution-end
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
