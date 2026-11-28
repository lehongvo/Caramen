// ==================================================================
// PRACTICE 0926: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0926_oncecell_memoized_fields.md
// Run the tests:          cargo test practice::p0926
// ==================================================================

//! # Problem 0926: Memoized Fields with `OnceCell`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::cell::OnceCell`, `get_or_init`, `set`/`get`, resetting caches with `take`, `Cell` counters, returning references into a cache
//! - **Source:** Original · [std: `OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html)
//!
//! ## Problem
//!
//! A `Document` computes expensive derived data (a word count, a word index) only when first asked
//! and then reuses it, even though the getters take `&self`. `OnceCell<T>` is made for this: it is
//! empty until the first `get_or_init(|| ...)` stores a value, and from then on it hands out
//! references to that value.
//!
//! Implement `Document` (choose the private fields):
//!
//! - `new(text: &str) -> Document`.
//! - `text(&self) -> &str`.
//! - `word_count(&self) -> usize`: the number of words, where a word is a whitespace-separated piece
//!   with leading and trailing non-alphanumeric characters removed, ignoring pieces that become
//!   empty (`"--"` is not a word). Computed once.
//! - `positions(&self, word: &str) -> &[usize]`: the 0-based indices, among those words, of every
//!   occurrence of `word`, compared in lowercase (`word` is lowercased too). The whole index
//!   (`HashMap<String, Vec<usize>>`) is built once, on the first call; absent words give `&[]`.
//! - `computations(&self) -> usize`: how many times a cached value was **computed** so far (the
//!   count and the index each add 1 when they are built).
//! - `set_title(&self, title: &str) -> Result<(), String>`: the title can be set only once; a second
//!   call returns `Err(rejected_title)`. `title(&self) -> Option<&str>`.
//! - `set_text(&mut self, text: &str)`: replaces the text and **clears** the count and the index
//!   (they will be recomputed on demand). The title is kept.
//!
//! ## Examples
//!
//! ```text
//! let doc = Document::new("The cat saw the other cat -- twice.");
//! doc.word_count()         => 7      computations() => 1
//! doc.word_count()         => 7      computations() => 1  (cached)
//! doc.positions("cat")     => [1, 5]
//! doc.positions("THE")     => [0, 3] computations() => 2
//! doc.set_title("Cats")    => Ok(());  doc.set_title("Dogs") => Err("Dogs")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `self.count.get_or_init(|| { self.computations.set(...); compute() })` returns `&usize`.
//! 2. `OnceCell::set` returns `Err(value)` if the cell was already full.
//! 3. `OnceCell::take(&mut self)` empties a cell; it needs `&mut self`, which `set_text` has.

use std::cell::{Cell, OnceCell};
use std::collections::HashMap;

pub struct Document {}

impl Document {
    pub fn new(text: &str) -> Document {
        todo!("Write your solution here")
    }

    pub fn text(&self) -> &str {
        todo!("Write your solution here")
    }

    pub fn word_count(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn positions(&self, word: &str) -> &[usize] {
        todo!("Write your solution here")
    }

    pub fn computations(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn set_title(&self, title: &str) -> Result<(), String> {
        todo!("Write your solution here")
    }

    pub fn title(&self) -> Option<&str> {
        todo!("Write your solution here")
    }

    pub fn set_text(&mut self, text: &str) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_is_computed_once() {
        let doc = Document::new("The cat saw the other cat -- twice.");
        assert_eq!(doc.computations(), 0);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.text(), "The cat saw the other cat -- twice.");
    }

    #[test]
    fn positions_use_a_lazily_built_index() {
        let doc = Document::new("The cat saw the other cat -- twice.");
        assert_eq!(doc.positions("cat"), &[1, 5]);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.positions("THE"), &[0, 3]);
        assert_eq!(doc.positions("twice"), &[6]);
        assert!(doc.positions("dog").is_empty());
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 2);
    }

    #[test]
    fn title_can_be_set_once() {
        let doc = Document::new("");
        assert_eq!(doc.title(), None);
        assert_eq!(doc.set_title("Cats"), Ok(()));
        assert_eq!(doc.set_title("Dogs"), Err("Dogs".to_string()));
        assert_eq!(doc.title(), Some("Cats"));
        assert_eq!(doc.word_count(), 0);
    }

    #[test]
    fn set_text_clears_the_caches() {
        let mut doc = Document::new("one two");
        doc.set_title("Numbers").unwrap();
        assert_eq!(doc.word_count(), 2);
        assert_eq!(doc.positions("two"), &[1]);
        doc.set_text("two two two");
        assert_eq!(doc.word_count(), 3);
        assert_eq!(doc.positions("two"), &[0, 1, 2]);
        assert_eq!(doc.computations(), 4);
        assert_eq!(doc.title(), Some("Numbers"));
    }

    #[test]
    fn large_document() {
        let text = "alpha beta, gamma! ".repeat(10_000);
        let doc = Document::new(&text);
        assert_eq!(doc.word_count(), 30_000);
        assert_eq!(doc.positions("gamma").len(), 10_000);
        assert_eq!(doc.positions("gamma")[9_999], 29_999);
    }
}
