//! # Problem 0825: Edit Distance
//!
//! - **Difficulty:** Medium
//! - **Topics:** Levenshtein distance, 2-D DP with rolling rows, `(0..=n).collect()`
//! - **Source:** [LeetCode 72. Edit Distance](https://leetcode.com/problems/edit-distance/)
//!
//! ## Problem
//!
//! Given two strings `word1` and `word2`, return the minimum number of single-character operations
//! that turn `word1` into `word2`. The allowed operations are:
//!
//! - **insert** a character;
//! - **delete** a character;
//! - **replace** a character with another one.
//!
//! Implement `Solution::min_distance(word1: String, word2: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word1 = "horse", word2 = "ros"
//! Output: 3
//! Explanation: horse -> rorse (replace 'h') -> rose (delete 'r') -> ros (delete 'e').
//!
//! Input:  word1 = "intention", word2 = "execution"
//! Output: 5
//!
//! Input:  word1 = "", word2 = "abc"
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `0 <= word1.len(), word2.len() <= 500`
//! - Both strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Let `D(i, j)` be the distance between `word1[..i]` and `word2[..j]`. Turning a prefix into
//!    the empty string (or back) costs its length: `D(i, 0) = i`, `D(0, j) = j`.
//! 2. If the last characters are equal, `D(i, j) = D(i - 1, j - 1)`. Otherwise it is
//!    `1 + min(replace, delete, insert)` = `1 + min(D(i-1, j-1), D(i-1, j), D(i, j-1))`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the classic DP above, stored as two rows. `prev` is row `i - 1`; `cur[0] = i`, and
//! each `cur[j + 1]` is computed from `prev[j]` (diagonal), `prev[j + 1]` (above) and `cur[j]`
//! (left).
//!
//! **Complexity:** `O(len1 * len2)` time, `O(len2)` space.
//!
//! **Rust concepts:**
//! - `(0..=n as i32).collect::<Vec<i32>>()` builds the first row `0, 1, ..., n`.
//! - `min` chained as `a.min(b).min(c)`; no need for a helper.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // @solution-start
        let (a, b) = (word1.as_bytes(), word2.as_bytes());
        let mut prev: Vec<i32> = (0..=b.len() as i32).collect();
        let mut cur = vec![0_i32; b.len() + 1];
        for (i, &x) in a.iter().enumerate() {
            cur[0] = i as i32 + 1;
            for (j, &y) in b.iter().enumerate() {
                cur[j + 1] = if x == y {
                    prev[j]
                } else {
                    1 + prev[j].min(prev[j + 1]).min(cur[j])
                };
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[b.len()]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dist(a: &str, b: &str) -> i32 {
        Solution::min_distance(a.to_string(), b.to_string())
    }

    #[test]
    fn example_horse_ros() {
        assert_eq!(dist("horse", "ros"), 3);
    }

    #[test]
    fn example_intention_execution() {
        assert_eq!(dist("intention", "execution"), 5);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(dist("", "abc"), 3);
        assert_eq!(dist("abc", ""), 3);
        assert_eq!(dist("", ""), 0);
    }

    #[test]
    fn classic_kitten_sitting() {
        assert_eq!(dist("kitten", "sitting"), 3);
        assert_eq!(dist("same", "same"), 0);
    }

    #[test]
    fn long_completely_different() {
        assert_eq!(dist(&"a".repeat(500), &"b".repeat(500)), 500);
    }
}
