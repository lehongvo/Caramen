// ==================================================================
// PRACTICE 0314: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0314_sort_the_people.md
// Run the tests:          cargo test practice::p0314
// ==================================================================

//! # Problem 0314: Sort the People
//!
//! - **Difficulty:** Easy
//! - **Topics:** `zip`, `Vec<(i32, String)>`, `sort_unstable_by_key`, `std::cmp::Reverse`
//! - **Source:** [LeetCode 2418. Sort the People](https://leetcode.com/problems/sort-the-people/)
//!
//! ## Problem
//!
//! You are given two vectors of the same length: `names[i]` is the name of person `i` and
//! `heights[i]` is that person's height. All heights are **distinct**. Return the names ordered
//! from the tallest person to the shortest.
//!
//! Implement `Solution::sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  names = ["Mary", "John", "Emma"], heights = [180, 165, 170]
//! Output: ["Mary", "Emma", "John"]
//!
//! Input:  names = ["Alice", "Bob", "Bob"], heights = [155, 185, 150]
//! Output: ["Bob", "Alice", "Bob"]     (names may repeat; heights do not)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= names.len() == heights.len() <= 1000`
//! - `1 <= names[i].len() <= 20`, `1 <= heights[i] <= 10^5`, heights are distinct
//!
//! ## Hints
//!
//! 1. Keep each name together with its height: `heights.into_iter().zip(names)` gives pairs.
//! 2. Sorting by `Reverse(height)` sorts from largest to smallest.

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::sort_people(strings(&["Mary", "John", "Emma"]), vec![180, 165, 170]),
            strings(&["Mary", "Emma", "John"])
        );
    }

    #[test]
    fn example_two_repeated_names() {
        assert_eq!(
            Solution::sort_people(strings(&["Alice", "Bob", "Bob"]), vec![155, 185, 150]),
            strings(&["Bob", "Alice", "Bob"])
        );
    }

    #[test]
    fn single_person() {
        assert_eq!(
            Solution::sort_people(strings(&["Solo"]), vec![1]),
            strings(&["Solo"])
        );
    }

    #[test]
    fn already_ascending_becomes_descending() {
        let names: Vec<String> = (0..1000).map(|i| format!("p{i}")).collect();
        let heights: Vec<i32> = (1..=1000).collect();
        let result = Solution::sort_people(names, heights);
        assert_eq!(result[0], "p999");
        assert_eq!(result[999], "p0");
    }
}
