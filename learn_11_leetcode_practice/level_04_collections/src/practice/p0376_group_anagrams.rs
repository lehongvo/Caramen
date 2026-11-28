// ==================================================================
// PRACTICE 0376: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0376_group_anagrams.md
// Run the tests:          cargo test practice::p0376
// ==================================================================

//! # Problem 0376: Group Anagrams
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<Vec<u8>, Vec<String>>` with a computed key, `entry().or_default()`, `into_values`
//! - **Source:** [LeetCode 49. Group Anagrams](https://leetcode.com/problems/group-anagrams/)
//!
//! ## Problem
//!
//! Given a list of strings `strs`, group together the strings that are **anagrams** of each other
//! (they contain the same letters the same number of times, like `"eat"`, `"tea"` and `"ate"`).
//! Return the groups as a list of lists. The groups, and the strings inside each group, may be in
//! **any order**.
//!
//! Implement `Solution::group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
//! Output: [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]     (any order)
//!
//! Input:  strs = [""]
//! Output: [[""]]
//!
//! Input:  strs = ["a"]
//! Output: [["a"]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= strs.len() <= 10^4`
//! - `0 <= strs[i].len() <= 100`, lowercase English letters
//!
//! ## Hints
//!
//! 1. Anagrams become identical once their letters are sorted: `"eat"`, `"tea"` -> `"aet"`.
//! 2. Use that canonical form as a `HashMap` key and push each string into its key's group.
//! 3. `map.into_values()` hands over the groups without cloning them.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        let result = Solution::group_anagrams(strings(&["eat", "tea", "tan", "ate", "nat", "bat"]));
        assert_eq!(
            normalized(result),
            vec![
                strings(&["ate", "eat", "tea"]),
                strings(&["bat"]),
                strings(&["nat", "tan"])
            ]
        );
    }

    #[test]
    fn example_empty_string() {
        assert_eq!(
            Solution::group_anagrams(strings(&[""])),
            vec![strings(&[""])]
        );
    }

    #[test]
    fn example_single_letter() {
        assert_eq!(
            Solution::group_anagrams(strings(&["a"])),
            vec![strings(&["a"])]
        );
    }

    #[test]
    fn duplicates_and_different_counts() {
        let result = Solution::group_anagrams(strings(&["ab", "ba", "ab", "aab", "abb"]));
        assert_eq!(
            normalized(result),
            vec![
                strings(&["aab"]),
                strings(&["ab", "ab", "ba"]),
                strings(&["abb"])
            ]
        );
    }

    #[test]
    fn many_strings() {
        let words: Vec<String> = (0..10_000)
            .map(|i| {
                let base = ["listen", "silent", "enlist", "google", "gogole"][i % 5];
                base.to_string()
            })
            .collect();
        let groups = normalized(Solution::group_anagrams(words));
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 6_000);
        assert_eq!(groups[1].len(), 4_000);
    }
}
