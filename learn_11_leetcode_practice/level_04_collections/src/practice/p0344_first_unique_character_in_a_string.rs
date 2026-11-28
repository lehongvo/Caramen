// ==================================================================
// PRACTICE 0344: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0344_first_unique_character_in_a_string.md
// Run the tests:          cargo test practice::p0344
// ==================================================================

//! # Problem 0344: First Unique Character in a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** two passes: count with a `HashMap`, then scan in the original order; `position`
//! - **Source:** [LeetCode 387. First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/)
//!
//! ## Problem
//!
//! Given a string `s`, find the first character that occurs **exactly once** in `s` and return its
//! index. If every character repeats, return `-1`.
//!
//! Implement `Solution::first_uniq_char(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "leetcode"
//! Output: 0       ('l' occurs once)
//!
//! Input:  s = "loveleetcode"
//! Output: 2       ('l' and 'o' repeat; 'v' at index 2 is the first unique one)
//!
//! Input:  s = "aabb"
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. You cannot know whether a character is unique before you have seen the whole string, so
//!    count first.
//! 2. A `HashMap` has no order. Get the "first" by scanning the **string** again, not the map.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn example_three_none() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }

    #[test]
    fn single_and_last() {
        assert_eq!(Solution::first_uniq_char("z".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("aabbc".to_string()), 4);
    }

    #[test]
    fn large_string_unique_at_the_end() {
        let s = format!("{}q", "abcdefghijklmnoprstuvwxyz".repeat(3_999));
        assert_eq!(Solution::first_uniq_char(s), 99_975);
    }
}
