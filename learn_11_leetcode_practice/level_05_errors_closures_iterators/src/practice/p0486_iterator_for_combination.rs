// ==================================================================
// PRACTICE 0486: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0486_iterator_for_combination.md
// Run the tests:          cargo test practice::p0486
// ==================================================================

//! # Problem 0486: Iterator for Combination
//!
//! - **Difficulty:** Medium
//! - **Topics:** LeetCode design struct, next lexicographic combination of indexes, `rposition` on a range, `Option<Vec<usize>>` as iterator state
//! - **Source:** [LeetCode 1286. Iterator for Combination](https://leetcode.com/problems/iterator-for-combination/)
//!
//! ## Problem
//!
//! Design `CombinationIterator`:
//!
//! - `CombinationIterator::new(characters: String, combination_length: i32) -> Self`: `characters`
//!   contains **distinct** lowercase letters in **sorted** order;
//! - `next(&mut self) -> String`: the next combination of `combination_length` characters, in
//!   lexicographic order (each combination keeps the characters in their original order);
//! - `has_next(&self) -> bool`: whether another combination exists.
//!
//! LeetCode's template names the parameter `combinationLength` and declares `next(&self)`; Rust
//! style asks for snake case, and `next` needs `&mut self` because it advances the iterator. Both
//! changes are accepted by LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut it = CombinationIterator::new("abc".to_string(), 2);
//! it.next()     -> "ab"
//! it.has_next() -> true
//! it.next()     -> "ac"
//! it.has_next() -> true
//! it.next()     -> "bc"
//! it.has_next() -> false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= combination_length <= characters.len() <= 15`
//! - All characters are distinct lowercase letters, sorted.
//! - At most `10^4` calls; every call to `next` is valid.
//!
//! ## Hints
//!
//! 1. Represent a combination by the increasing indexes it uses: `[0, 1]`, `[0, 2]`, `[1, 2]` for
//!    the example.
//! 2. To get the next combination, find the **rightmost** index that can still grow (index `i` can
//!    grow while it is below `n - k + i`), increase it, and reset every index after it to the
//!    smallest possible values.
//! 3. Store the upcoming combination as `Option<Vec<usize>>`: `None` means "no more".

pub struct CombinationIterator {}

impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        todo!("Write your solution here")
    }

    // LeetCode requires this exact name; it is not `Iterator::next`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        todo!("Write your solution here")
    }

    pub fn has_next(&self) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all(chars: &str, k: i32) -> Vec<String> {
        let mut it = CombinationIterator::new(chars.to_string(), k);
        let mut out = Vec::new();
        while it.has_next() {
            out.push(it.next());
        }
        out
    }

    #[test]
    fn example_abc_choose_2() {
        let mut it = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!(it.next(), "ab");
        assert!(it.has_next());
        assert_eq!(it.next(), "ac");
        assert!(it.has_next());
        assert_eq!(it.next(), "bc");
        assert!(!it.has_next());
    }

    #[test]
    fn single_and_full_length() {
        assert_eq!(all("abcd", 1), vec!["a", "b", "c", "d"]);
        assert_eq!(all("abcd", 4), vec!["abcd"]);
    }

    #[test]
    fn three_of_five_in_order() {
        let combos = all("abcde", 3);
        assert_eq!(combos.len(), 10);
        assert_eq!(combos[0], "abc");
        assert_eq!(combos[1], "abd");
        assert_eq!(combos[9], "cde");
        assert!(combos.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn largest_input() {
        // C(15, 7) = 6435 combinations, each strictly larger than the previous one.
        let combos = all("abcdefghijklmno", 7);
        assert_eq!(combos.len(), 6435);
        assert_eq!(combos.last().map(String::as_str), Some("ijklmno"));
    }
}
