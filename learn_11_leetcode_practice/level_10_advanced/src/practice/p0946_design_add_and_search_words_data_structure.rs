// ==================================================================
// PRACTICE 0946: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0946_design_add_and_search_words_data_structure.md
// Run the tests:          cargo test practice::p0946
// ==================================================================

//! # Problem 0946: Design Add and Search Words Data Structure
//!
//! - **Difficulty:** Medium
//! - **Topics:** tries, wildcard search with recursive DFS over `&Node`, byte slices, `Iterator::any`
//! - **Source:** [LeetCode 211. Design Add and Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure/)
//!
//! ## Problem
//!
//! Build a dictionary of words that supports searching with **wildcards**: in a search pattern, the
//! character `'.'` matches any single letter.
//!
//! Implement the `WordDictionary` struct:
//!
//! - `WordDictionary::new() -> Self`: an empty dictionary.
//! - `add_word(&mut self, word: String)`: stores `word`.
//! - `search(&self, word: String) -> bool`: `true` if some stored word has exactly the same length as
//!   the pattern `word` and matches it letter by letter, where `'.'` matches any letter.
//!
//! ## Examples
//!
//! ```text
//! add_word("bad"); add_word("dad"); add_word("mad");
//! search("pad")   => false
//! search("bad")   => true
//! search(".ad")   => true
//! search("b..")   => true
//! search("b.")    => false   (lengths differ)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word.len() <= 25`
//! - Added words contain only lowercase letters; search patterns contain lowercase letters or `'.'`.
//! - A search pattern contains at most 2 dots.
//! - At most `10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Store the words in a trie (problem 0945).
//! 2. Search recursively: `matches(node, rest)` follows one child for a letter and tries all
//!    children for a `'.'`.
//! 3. The recursion ends when `rest` is empty: the answer is whether the node ends a word.

pub struct WordDictionary {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl WordDictionary {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_word(&mut self, word: String) {
        todo!("Write your solution here")
    }

    pub fn search(&self, word: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dictionary(words: &[&str]) -> WordDictionary {
        let mut dict = WordDictionary::new();
        for word in words {
            dict.add_word(word.to_string());
        }
        dict
    }

    #[test]
    fn example_sequence() {
        let dict = dictionary(&["bad", "dad", "mad"]);
        assert!(!dict.search("pad".to_string()));
        assert!(dict.search("bad".to_string()));
        assert!(dict.search(".ad".to_string()));
        assert!(dict.search("b..".to_string()));
        assert!(!dict.search("b.".to_string()));
    }

    #[test]
    fn lengths_must_match() {
        let dict = dictionary(&["a", "ab"]);
        assert!(dict.search(".".to_string()));
        assert!(dict.search("..".to_string()));
        assert!(!dict.search("...".to_string()));
        assert!(dict.search("a.".to_string()));
        assert!(!dict.search(".a".to_string()));
    }

    #[test]
    fn empty_dictionary() {
        let dict = WordDictionary::new();
        assert!(!dict.search("a".to_string()));
        assert!(!dict.search(".".to_string()));
    }

    #[test]
    fn many_words_with_wildcards() {
        let mut dict = WordDictionary::new();
        for a in b'a'..=b'z' {
            for b in b'a'..=b'z' {
                if (a + b) % 3 == 0 {
                    dict.add_word(String::from_utf8(vec![a, b, b'x']).unwrap());
                }
            }
        }
        assert!(dict.search("..x".to_string()));
        assert!(!dict.search("..y".to_string()));
        // 'a' + 'b' = 97 + 98 = 195, divisible by 3.
        assert!(dict.search("abx".to_string()));
        assert!(!dict.search("aax".to_string()));
        assert!(dict.search("a.x".to_string()));
    }
}
