//! # Problem 0179: Longest Common Prefix
//!
//! - **Difficulty:** Easy
//! - **Topics:** shrinking a borrowed `&str` prefix, `split_first`, `bytes().zip(..)`, `take_while`
//! - **Source:** [LeetCode 14. Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)
//!
//! ## Problem
//!
//! Return the longest string that is a prefix of **every** string in `strs`. If the strings have no
//! common prefix, return an empty string.
//!
//! Implement `Solution::longest_common_prefix(strs: Vec<String>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  strs = ["flower", "flow", "flight"]
//! Output: "fl"
//!
//! Input:  strs = ["dog", "racecar", "car"]
//! Output: ""
//!
//! Input:  strs = ["alone"]
//! Output: "alone"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= strs.len() <= 200`
//! - `0 <= strs[i].len() <= 200`
//! - Strings consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. Start with the whole first string as the candidate prefix (a `&str` borrowed from it).
//! 2. For each other string, count how many leading bytes match
//!    (`prefix.bytes().zip(s.bytes()).take_while(|(a, b)| a == b).count()`) and shrink the candidate
//!    to that length.
//! 3. Allocate the answer only once, at the end.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the common prefix of all strings is obtained by repeatedly intersecting a candidate with
//! the next string. The candidate is a slice of the first string, so shrinking it is just changing
//! a length; one `to_string()` at the end produces the owned answer.
//!
//! **Complexity:** `O(total length)` time, `O(1)` extra space besides the answer.
//!
//! **Rust concepts:**
//! - `split_first()` returns `Option<(&T, &[T])>`: the first element and the rest, borrowed.
//! - `let ... else` handles the empty case (impossible under the constraints, but cheap to cover).
//! - `&prefix[..common]` is safe because `common` counts matching ASCII bytes.
//! - `zip` stops at the shorter input, so different lengths need no special case.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // @solution-start
        let Some((first, rest)) = strs.split_first() else {
            return String::new();
        };
        let mut prefix: &str = first;
        for s in rest {
            let common = prefix
                .bytes()
                .zip(s.bytes())
                .take_while(|(a, b)| a == b)
                .count();
            prefix = &prefix[..common];
        }
        prefix.to_string()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(strs: &[&str]) -> String {
        Solution::longest_common_prefix(strs.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn example_fl() {
        assert_eq!(run(&["flower", "flow", "flight"]), "fl");
    }

    #[test]
    fn example_nothing_in_common() {
        assert_eq!(run(&["dog", "racecar", "car"]), "");
    }

    #[test]
    fn example_single_string() {
        assert_eq!(run(&["alone"]), "alone");
    }

    #[test]
    fn one_string_is_a_prefix_of_the_others() {
        assert_eq!(run(&["interview", "inter", "internet"]), "inter");
    }

    #[test]
    fn empty_string_in_the_list() {
        assert_eq!(run(&["abc", "", "abd"]), "");
    }

    #[test]
    fn largest_input() {
        let base = "q".repeat(150);
        let strs: Vec<String> = (0..200)
            .map(|i| format!("{base}{}", "z".repeat(i % 50)))
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), base);
    }
}
