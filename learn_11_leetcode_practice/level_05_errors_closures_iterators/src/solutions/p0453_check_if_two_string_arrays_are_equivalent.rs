//! # Problem 0453: Check If Two String Arrays are Equivalent
//!
//! - **Difficulty:** Easy
//! - **Topics:** `flat_map` over `chars`, `Iterator::eq`, lazy comparison without allocation
//! - **Source:** [LeetCode 1662. Check If Two String Arrays are Equivalent](https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/)
//!
//! ## Problem
//!
//! Two lists of strings are *equivalent* when concatenating the strings of each list, in order,
//! produces the same text. Given `word1` and `word2`, return whether they are equivalent.
//!
//! Implement `Solution::array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word1 = ["ab", "c"], word2 = ["a", "bc"]
//! Output: true
//! Explanation: both concatenate to "abc".
//!
//! Input:  word1 = ["a", "cb"], word2 = ["ab", "c"]
//! Output: false
//!
//! Input:  word1 = ["abc", "d", "defg"], word2 = ["abcddefg"]
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word1.len(), word2.len() <= 10^3`
//! - `1 <= word1[i].len(), word2[i].len() <= 10^3`
//! - The total length of each list's strings is at most `10^3`.
//! - All strings contain lowercase English letters.
//!
//! ## Hints
//!
//! 1. `word1.concat() == word2.concat()` works but builds two new strings.
//! 2. `word1.iter().flat_map(|s| s.chars())` is an iterator over all characters of the list.
//! 3. Two iterators can be compared element by element with `Iterator::eq`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** view each list as one long stream of characters with `flat_map`, then compare the two
//! streams with `eq`, which stops at the first difference.
//!
//! **Complexity:** `O(L)` time where `L` is the total length; `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `flat_map(f)` maps each element to an iterator and flattens the results into one iterator.
//! - `Iterator::eq` compares two iterators lazily: different lengths or the first different item
//!   make it return `false` immediately.
//! - Nothing is allocated: the characters are read straight from the original strings.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        // @solution-start
        word1
            .iter()
            .flat_map(|s| s.chars())
            .eq(word2.iter().flat_map(|s| s.chars()))
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(a: &[&str], b: &[&str]) -> bool {
        let to_vec = |v: &[&str]| v.iter().map(|s| s.to_string()).collect();
        Solution::array_strings_are_equal(to_vec(a), to_vec(b))
    }

    #[test]
    fn example_equal() {
        assert!(run(&["ab", "c"], &["a", "bc"]));
    }

    #[test]
    fn example_different_order() {
        assert!(!run(&["a", "cb"], &["ab", "c"]));
    }

    #[test]
    fn example_one_long_string() {
        assert!(run(&["abc", "d", "defg"], &["abcddefg"]));
    }

    #[test]
    fn prefix_is_not_equal() {
        assert!(!run(&["abc"], &["ab"]));
        assert!(!run(&["ab"], &["abc"]));
        assert!(run(&["a"], &["a"]));
    }

    #[test]
    fn long_inputs() {
        let many: Vec<&str> = vec!["x"; 1000];
        let one = "x".repeat(1000);
        assert!(run(&many, &[one.as_str()]));
        let mut almost = "x".repeat(999);
        almost.push('y');
        assert!(!run(&many, &[almost.as_str()]));
    }
}
