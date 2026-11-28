//! # Problem 0800: Palindrome Partitioning
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking over cut positions, precomputed palindrome table, `&str` slicing, `Vec<Vec<String>>`
//! - **Source:** [LeetCode 131. Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)
//!
//! ## Problem
//!
//! Given a string `s` of lowercase English letters, split it into consecutive non-empty parts so
//! that **every part is a palindrome**. Return all possible such partitions, in any order; each
//! partition is a vector of the parts in their original left-to-right order.
//!
//! Implement `Solution::partition(s: String) -> Vec<Vec<String>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aab"
//! Output: [["a", "a", "b"], ["aa", "b"]]
//!
//! Input:  s = "a"
//! Output: [["a"]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 16`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Choose the first part: any palindromic prefix. Then partition the rest of the string the same
//!    way (backtracking).
//! 2. The same "is `s[i..=j]` a palindrome?" question is asked many times. Precompute a table:
//!    `s[i..=j]` is a palindrome when `s[i] == s[j]` and `s[i + 1..j]` is a palindrome.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking over cut positions + palindrome table: first fill `pal[i][j]` ("is
//! `s[i..=j]` a palindrome?") from the end of the string towards the start, so that
//! `pal[i + 1][j - 1]` is ready when `pal[i][j]` needs it; ranges of length 1 or 2 only need
//! `s[i] == s[j]`. Then `backtrack(start)` tries every end `end >= start` with `pal[start][end]`:
//! push the part `s[start..=end]`, partition the rest from `end + 1`, pop. Reaching the end of the
//! string means `current` is a complete partition.
//!
//! **Complexity:** `O(n * 2^n)` time in the worst case (`"aaaa..."` has `2^(n - 1)` partitions),
//! `O(n^2)` space for the table.
//!
//! **Rust concepts:**
//! - `&s[start..=end]` slices a `&str` by **byte** range; it is valid here because the input is
//!   ASCII (slicing in the middle of a multi-byte character would panic).
//! - `.to_string()` copies the slice into an owned `String` for the result.
//! - The table is `Vec<Vec<bool>>`, passed to the helper as `&[Vec<bool>]`.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(
    s: &str,
    pal: &[Vec<bool>],
    start: usize,
    current: &mut Vec<String>,
    result: &mut Vec<Vec<String>>,
) {
    if start == s.len() {
        result.push(current.clone());
        return;
    }
    for end in start..s.len() {
        if pal[start][end] {
            current.push(s[start..=end].to_string());
            backtrack(s, pal, end + 1, current, result);
            current.pop();
        }
    }
}
// @solution-end

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        // @solution-start
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut pal = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                pal[i][j] = bytes[i] == bytes[j] && (j - i < 2 || pal[i + 1][j - 1]);
            }
        }
        let mut result = Vec::new();
        backtrack(&s, &pal, 0, &mut Vec::new(), &mut result);
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<Vec<String>> {
        let mut result = Solution::partition(s.to_string());
        result.sort(); // any order is accepted
        result
    }

    fn parts(list: &[&[&str]]) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = list
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect())
            .collect();
        result.sort();
        result
    }

    #[test]
    fn example_aab() {
        assert_eq!(run("aab"), parts(&[&["a", "a", "b"], &["aa", "b"]]));
    }

    #[test]
    fn example_single_letter() {
        assert_eq!(run("a"), parts(&[&["a"]]));
    }

    #[test]
    fn odd_palindrome() {
        assert_eq!(run("aba"), parts(&[&["a", "b", "a"], &["aba"]]));
    }

    #[test]
    fn repeated_letter_gives_every_split() {
        // Every one of the 3 gaps of "aaaa" can be cut or not: 2^3 = 8 partitions.
        assert_eq!(run("aaaa").len(), 8);
    }

    #[test]
    fn longest_inputs() {
        assert_eq!(run("abcdefghijklmnop").len(), 1);
        let many = run(&"a".repeat(16));
        assert_eq!(many.len(), 1 << 15);
        assert!(many.iter().all(|p| p.concat() == "a".repeat(16)));
    }
}
