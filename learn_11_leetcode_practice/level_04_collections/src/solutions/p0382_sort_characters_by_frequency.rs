//! # Problem 0382: Sort Characters By Frequency
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<char, usize>` counts into a `Vec<(char, usize)>`, `sort_unstable_by`, `String` building with `extend` + `repeat_n`
//! - **Source:** [LeetCode 451. Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency/)
//!
//! ## Problem
//!
//! Given a string `s`, rearrange its characters so that they appear in **decreasing order of
//! frequency**: a character that occurs more often comes before one that occurs less often, and all
//! copies of a character are next to each other. If several answers exist (characters with equal
//! frequency), return any of them.
//!
//! Implement `Solution::frequency_sort(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "tree"
//! Output: "eert"      ("eetr" is also accepted; "rete" is not, the e's must be together)
//!
//! Input:  s = "cccaaa"
//! Output: "aaaccc"    ("cccaaa" is also accepted)
//!
//! Input:  s = "Aabb"
//! Output: "bbAa"      ("bbaA" is also accepted; 'A' and 'a' are different characters)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 5 * 10^5`
//! - `s` contains English letters (both cases) and digits.
//!
//! ## Hints
//!
//! 1. Count each character, then sort the distinct characters by count, largest first.
//! 2. Build the answer by appending each character `count` times.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** count characters in a `HashMap`, move the `(char, count)` pairs into a `Vec`, sort
//! them by count descending (ties by character, to make the output deterministic), and append each
//! character `count` times.
//!
//! **Complexity:** `O(n + d log d)` time for `d` distinct characters (at most 62), `O(n)` space.
//!
//! **Rust concepts:**
//! - `String::with_capacity(s.len())` allocates the output once (all characters are ASCII here).
//! - `result.extend(std::iter::repeat_n(c, count))` appends `count` copies of `c`.
//! - Because several answers are valid, the tests use a checker: the output must be a permutation
//!   of `s`, each character's copies must be contiguous, and the group sizes must not increase.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        // @solution-start
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut pairs: Vec<(char, usize)> = counts.into_iter().collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let mut result = String::with_capacity(s.len());
        for (c, count) in pairs {
            result.extend(std::iter::repeat_n(c, count));
        }
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A valid answer is a permutation of `s` made of contiguous groups of non-increasing size.
    fn is_valid(s: &str, result: &str) -> bool {
        let mut a: Vec<char> = s.chars().collect();
        let mut b: Vec<char> = result.chars().collect();
        a.sort_unstable();
        b.sort_unstable();
        if a != b {
            return false;
        }
        let chars: Vec<char> = result.chars().collect();
        let groups: Vec<&[char]> = chars.chunk_by(|x, y| x == y).collect();
        let mut firsts: Vec<char> = groups.iter().map(|g| g[0]).collect();
        let distinct_groups = firsts.len();
        firsts.sort_unstable();
        firsts.dedup();
        firsts.len() == distinct_groups && groups.windows(2).all(|w| w[0].len() >= w[1].len())
    }

    fn check(s: &str) {
        let result = Solution::frequency_sort(s.to_string());
        assert!(is_valid(s, &result), "invalid answer {result:?} for {s:?}");
    }

    #[test]
    fn example_tree() {
        assert!(!is_valid("tree", "rete"));
        assert!(is_valid("tree", "eetr"));
        check("tree");
    }

    #[test]
    fn example_cccaaa() {
        check("cccaaa");
    }

    #[test]
    fn example_case_sensitive() {
        assert!(!is_valid("Aabb", "Aabb"));
        check("Aabb");
    }

    #[test]
    fn digits_and_single_char() {
        check("1");
        check("2a2a2");
        assert_eq!(Solution::frequency_sort("2a2a2".to_string()), "222aa");
    }

    #[test]
    fn long_string() {
        // 300_000 characters = 50_000 copies of "xyyzzz": z 150_000, y 100_000, x 50_000 times.
        let s: String = (0..300_000)
            .map(|i| ['x', 'y', 'y', 'z', 'z', 'z'][i % 6])
            .collect();
        let result = Solution::frequency_sort(s.clone());
        assert!(is_valid(&s, &result));
        let expected = format!(
            "{}{}{}",
            "z".repeat(150_000),
            "y".repeat(100_000),
            "x".repeat(50_000)
        );
        assert_eq!(result, expected);
    }
}
