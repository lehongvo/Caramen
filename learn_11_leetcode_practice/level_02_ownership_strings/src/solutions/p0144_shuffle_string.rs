//! # Problem 0144: Shuffle String
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Vec<char>` buffer, `zip`, `i32` to `usize` indices, collecting into a `String`
//! - **Source:** [LeetCode 1528. Shuffle String](https://leetcode.com/problems/shuffle-string/)
//!
//! ## Problem
//!
//! You get a string `s` and a vector `indices` of the same length. The character at position `i` of
//! `s` must move to position `indices[i]` of the result. `indices` is a permutation of
//! `0..s.len()`, so every position of the result receives exactly one character.
//!
//! Implement `Solution::restore_string(s: String, indices: Vec<i32>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "codeleet", indices = [4, 5, 6, 7, 0, 2, 1, 3]
//! Output: "leetcode"
//! Explanation: 'c' goes to 4, 'o' to 5, 'd' to 6, 'e' to 7, 'l' to 0, 'e' to 2, 'e' to 1, 't' to 3.
//!
//! Input:  s = "abc", indices = [0, 1, 2]
//! Output: "abc"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() == indices.len() <= 100`
//! - `s` consists of lowercase English letters.
//! - `indices` is a permutation of `0..s.len()`.
//!
//! ## Hints
//!
//! 1. A `String` cannot be written at arbitrary positions, but a `Vec<char>` can:
//!    start from `vec![' '; n]`.
//! 2. `s.chars().zip(&indices)` pairs each character with its target position.
//! 3. `buffer.into_iter().collect::<String>()` turns the `Vec<char>` back into a `String`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** use a character buffer of the right length, drop every character into its target slot,
//! and collect the buffer into a `String`.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - Strings are UTF-8 byte sequences, so Rust offers no `s[i] = c`; working in a `Vec<char>` (or a
//!   `Vec<u8>` for pure ASCII) and converting back is the usual approach.
//! - `zip(&indices)` iterates the vector by reference, yielding `&i32`; `idx as usize` converts the
//!   (guaranteed non-negative) index.
//! - `FromIterator<char> for String` makes `collect()` build the final `String`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        // @solution-start
        let mut buffer = vec![' '; indices.len()];
        for (c, &idx) in s.chars().zip(&indices) {
            buffer[idx as usize] = c;
        }
        buffer.into_iter().collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_leetcode() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode"
        );
    }

    #[test]
    fn example_identity() {
        assert_eq!(Solution::restore_string("abc".to_string(), vec![0, 1, 2]), "abc");
    }

    #[test]
    fn reversed_permutation() {
        assert_eq!(Solution::restore_string("abcd".to_string(), vec![3, 2, 1, 0]), "dcba");
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::restore_string("z".to_string(), vec![0]), "z");
    }

    #[test]
    fn longest_input_rotation() {
        let s: String = (0..100).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let indices: Vec<i32> = (0..100).map(|i| (i + 1) % 100).collect();
        let result = Solution::restore_string(s.clone(), indices);
        // Every character moved one step to the right, the last one wrapped to the front.
        assert_eq!(&result[1..], &s[..99]);
        assert_eq!(&result[..1], &s[99..]);
    }
}
