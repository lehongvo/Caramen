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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the sorted bytes of a string identify its anagram class. Group the strings in a
//! `HashMap` keyed by that sorted form, then return the map's values.
//!
//! **Complexity:** `O(n * L log L)` time for `n` strings of length up to `L`, `O(n * L)` space.
//! (A `[u8; 26]` letter-count key avoids the sort and gives `O(n * L)`.)
//!
//! **Rust concepts:**
//! - Any type that is `Hash + Eq` can be a key, including `Vec<u8>` and arrays like `[u8; 26]`.
//! - `entry(key).or_default().push(s)` creates the group on first use; `s` is moved into the map,
//!   so no string is cloned (the key is a separate, sorted copy of the bytes).
//! - `into_values()` (stable since Rust 1.54) consumes the map and yields the `Vec<String>`s.
//! - The output order depends on the map, so the tests sort every group and then the list of groups.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // @solution-start
        let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = s.as_bytes().to_vec();
            key.sort_unstable();
            groups.entry(key).or_default().push(s);
        }
        groups.into_values().collect()
        // @solution-end
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
