// ==================================================================
// PRACTICE 0729: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0729_find_smallest_letter_greater_than_target.md
// Run the tests:          cargo test practice::p0729
// ==================================================================

//! # Problem 0729: Find Smallest Letter Greater Than Target
//!
//! - **Difficulty:** Easy
//! - **Topics:** binary search for an upper bound, wrapping around, `char` comparison
//! - **Source:** [LeetCode 744. Find Smallest Letter Greater Than Target](https://leetcode.com/problems/find-smallest-letter-greater-than-target/)
//!
//! ## Problem
//!
//! You are given a vector `letters` of lowercase letters sorted in non-decreasing order (it
//! contains at least two **different** letters) and a lowercase letter `target`. Return the
//! smallest letter in `letters` that is **strictly greater** than `target`. If no letter is greater,
//! wrap around and return `letters[0]`.
//!
//! Implement `Solution::next_greatest_letter(letters: Vec<char>, target: char) -> char`.
//!
//! ## Examples
//!
//! ```text
//! Input:  letters = ['c', 'f', 'j'], target = 'a'
//! Output: 'c'
//!
//! Input:  letters = ['c', 'f', 'j'], target = 'c'
//! Output: 'f'           (strictly greater: 'c' itself does not count)
//!
//! Input:  letters = ['x', 'x', 'y', 'y'], target = 'z'
//! Output: 'x'           (nothing is greater than 'z', so wrap around)
//! ```
//!
//! ## Constraints
//!
//! - `2 <= letters.len() <= 10^4`
//! - `letters[i]` and `target` are lowercase English letters.
//! - `letters` is sorted in non-decreasing order and contains at least two different letters.
//!
//! ## Hints
//!
//! 1. You are looking for the first position whose letter is `> target`: an "upper bound".
//! 2. Every letter `<= target` is to the left of that position and every letter `> target` is at
//!    or to the right of it. Binary search for the boundary.

pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_before_everything() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
    }

    #[test]
    fn example_equal_letter_is_skipped() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
    }

    #[test]
    fn example_wrap_around() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
            'x'
        );
    }

    #[test]
    fn target_equals_last_letter() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'),
            'c'
        );
    }

    #[test]
    fn duplicates_of_the_target() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['a', 'b', 'b', 'b', 'c'], 'b'),
            'c'
        );
    }

    #[test]
    fn long_input() {
        let mut letters = vec!['a'; 5_000];
        letters.extend(vec!['m'; 4_999]);
        letters.push('z');
        assert_eq!(Solution::next_greatest_letter(letters.clone(), 'a'), 'm');
        assert_eq!(Solution::next_greatest_letter(letters.clone(), 'm'), 'z');
        assert_eq!(Solution::next_greatest_letter(letters, 'z'), 'a');
    }
}
