//! # Problem 0175: Reverse String II
//!
//! - **Difficulty:** Easy
//! - **Topics:** `chunks_mut` on a byte buffer, reversing a sub-slice `chunk[..end].reverse()`, `min`
//! - **Source:** [LeetCode 541. Reverse String II](https://leetcode.com/problems/reverse-string-ii/)
//!
//! ## Problem
//!
//! Given a string `s` and an integer `k`, cut `s` into consecutive blocks of `2k` characters (the
//! last block may be shorter). In every block, reverse the first `k` characters and leave the rest
//! as they are. If a block has fewer than `k` characters, reverse all of them.
//!
//! Implement `Solution::reverse_str(s: String, k: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "abcdefg", k = 2
//! Output: "bacdfeg"        (blocks "abcd" and "efg": reverse "ab" and "ef")
//!
//! Input:  s = "abcd", k = 2
//! Output: "bacd"
//!
//! Input:  s = "abc", k = 5
//! Output: "cba"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^4`
//! - `s` consists of lowercase English letters.
//! - `1 <= k <= 10^4`
//!
//! ## Hints
//!
//! 1. `bytes.chunks_mut(2 * k)` hands you each block as a `&mut [u8]`.
//! 2. Reverse `chunk[..k.min(chunk.len())]`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the rules talk about blocks of `2k`, which is exactly what `chunks_mut(2 * k)` produces.
//! In each block, reverse the prefix of length `min(k, block length)`. Everything happens in the
//! string's own buffer.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `chunks_mut(n)` yields non-overlapping mutable sub-slices; the last one may be shorter.
//! - Range-indexing a mutable slice (`chunk[..end]`) gives a smaller mutable slice to call
//!   `reverse()` on.
//! - The input is ASCII, so reversing bytes is the same as reversing characters.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        // @solution-start
        let k = k as usize;
        let mut bytes = s.into_bytes();
        for chunk in bytes.chunks_mut(2 * k) {
            let end = k.min(chunk.len());
            chunk[..end].reverse();
        }
        String::from_utf8(bytes).expect("reversing ASCII bytes keeps the text valid UTF-8")
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, k: i32) -> String {
        Solution::reverse_str(s.to_string(), k)
    }

    #[test]
    fn example_two_blocks() {
        assert_eq!(run("abcdefg", 2), "bacdfeg");
    }

    #[test]
    fn example_one_full_block() {
        assert_eq!(run("abcd", 2), "bacd");
    }

    #[test]
    fn example_k_larger_than_string() {
        assert_eq!(run("abc", 5), "cba");
    }

    #[test]
    fn k_equal_to_one_changes_nothing() {
        assert_eq!(run("abcdef", 1), "abcdef");
    }

    #[test]
    fn last_block_between_k_and_2k() {
        assert_eq!(run("abcdefgh", 3), "cbadefhg");
    }

    #[test]
    fn largest_input() {
        let s = "ab".repeat(5_000);
        assert_eq!(run(&s, 1), s);
        // Blocks of 4 are "abab"; reversing the first 2 gives "baab".
        assert_eq!(run(&s, 2), "baab".repeat(2_500));
    }
}
