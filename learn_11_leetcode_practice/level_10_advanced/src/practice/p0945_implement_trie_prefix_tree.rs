// ==================================================================
// PRACTICE 0945: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0945_implement_trie_prefix_tree.md
// Run the tests:          cargo test practice::p0945
// ==================================================================

//! # Problem 0945: Implement Trie (Prefix Tree)
//!
//! - **Difficulty:** Medium
//! - **Topics:** tries, `[Option<Box<Node>>; 26]`, `Default` for arrays of `Option`, walking a tree with `&` and `&mut` references
//! - **Source:** [LeetCode 208. Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)
//!
//! ## Problem
//!
//! A **trie** stores a set of words as a tree of characters: every path from the root spells a
//! prefix, and nodes are marked where a complete word ends. Lookups cost `O(length of the word)`,
//! independent of how many words are stored.
//!
//! Implement the `Trie` struct:
//!
//! - `Trie::new() -> Self`: an empty trie.
//! - `insert(&mut self, word: String)`: adds `word` (inserting it twice changes nothing).
//! - `search(&self, word: String) -> bool`: `true` if `word` was inserted.
//! - `starts_with(&self, prefix: String) -> bool`: `true` if some inserted word starts with `prefix`
//!   (a word is a prefix of itself).
//!
//! ## Examples
//!
//! ```text
//! let mut trie = Trie::new();
//! trie.insert("apple")
//! trie.search("apple")     => true
//! trie.search("app")       => false
//! trie.starts_with("app")  => true
//! trie.insert("app")
//! trie.search("app")       => true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word.len(), prefix.len() <= 2000`
//! - Words and prefixes consist only of lowercase English letters.
//! - At most `3 * 10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Each node has 26 child slots, one per letter; `(b - b'a') as usize` is the slot of byte `b`.
//! 2. `insert` walks down with a `&mut Node`, creating missing children with
//!    `get_or_insert_with(Box::default)`.
//! 3. `search` and `starts_with` share a helper that returns the node reached by a prefix, if any.

pub struct Trie {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Trie {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn insert(&mut self, word: String) {
        todo!("Write your solution here")
    }

    pub fn search(&self, word: String) -> bool {
        todo!("Write your solution here")
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(text: &str) -> String {
        text.to_string()
    }

    #[test]
    fn example_sequence() {
        let mut trie = Trie::new();
        trie.insert(s("apple"));
        assert!(trie.search(s("apple")));
        assert!(!trie.search(s("app")));
        assert!(trie.starts_with(s("app")));
        trie.insert(s("app"));
        assert!(trie.search(s("app")));
    }

    #[test]
    fn prefixes_are_not_words() {
        let mut trie = Trie::new();
        trie.insert(s("banana"));
        trie.insert(s("band"));
        assert!(trie.starts_with(s("ban")));
        assert!(trie.starts_with(s("banana")));
        assert!(!trie.starts_with(s("bananas")));
        assert!(!trie.search(s("ban")));
        assert!(!trie.search(s("bananas")));
        assert!(!trie.starts_with(s("c")));
    }

    #[test]
    fn duplicates_and_single_letters() {
        let mut trie = Trie::new();
        trie.insert(s("a"));
        trie.insert(s("a"));
        assert!(trie.search(s("a")));
        assert!(!trie.search(s("b")));
        assert!(trie.starts_with(s("a")));
    }

    #[test]
    fn many_words_and_long_word() {
        let mut trie = Trie::new();
        for i in 0..2_000_u32 {
            let word: String = i
                .to_string()
                .bytes()
                .map(|d| char::from(b'a' + (d - b'0')))
                .collect();
            trie.insert(word);
        }
        assert!(trie.search(s("bjjj"))); // 1999
        assert!(!trie.search(s("caaa"))); // 2000 was not inserted
        assert!(trie.starts_with(s("bj")));
        let long = "z".repeat(2_000);
        trie.insert(long.clone());
        assert!(trie.search(long));
        assert!(trie.starts_with("z".repeat(1_999)));
    }
}
