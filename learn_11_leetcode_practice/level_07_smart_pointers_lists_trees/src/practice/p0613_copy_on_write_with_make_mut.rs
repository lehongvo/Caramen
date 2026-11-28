// ==================================================================
// PRACTICE 0613: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0613_copy_on_write_with_make_mut.md
// Run the tests:          cargo test practice::p0613
// ==================================================================

//! # Problem 0613: Copy-on-Write Document
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Rc::make_mut`, clone-on-write, `Rc::ptr_eq`, cheap snapshots
//! - **Source:** Original · [`Rc::make_mut`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.make_mut) · [The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html)
//!
//! ## Problem
//!
//! An editor keeps an undo history by taking a snapshot of the document after every change. Copying
//! all lines for each snapshot would be slow, so the document stores its lines behind an `Rc`:
//! a snapshot is just another handle. The lines are copied only when someone **writes** to a
//! document whose storage is shared (clone-on-write).
//!
//! Implement `Document` (its fields are up to you; an `Rc<Vec<String>>` is the intended choice):
//!
//! - `Document::new() -> Document` and `Document::from_lines(lines: &[&str]) -> Document`;
//! - `snapshot(&self) -> Document`: an `O(1)` copy that shares storage with `self`;
//! - `lines(&self) -> &[String]`;
//! - `push_line(&mut self, line: &str)`: append a line;
//! - `set_line(&mut self, index: usize, line: &str) -> bool`: replace a line and return `true`, or
//!   return `false` if `index` is out of range. A failed `set_line` must **not** copy anything;
//! - `shares_storage_with(&self, other: &Document) -> bool`: `true` when both use the same
//!   allocation.
//!
//! Writing to one document must never change any other document or snapshot. Writing to a
//! document whose storage is not shared must not copy the lines.
//!
//! ## Examples
//!
//! ```text
//! let mut doc = Document::from_lines(&["a", "b"]);
//! let snap = doc.snapshot();
//! doc.shares_storage_with(&snap) -> true
//! doc.push_line("c");                     // copies once, because `snap` shares the lines
//! doc.lines()  -> ["a", "b", "c"]
//! snap.lines() -> ["a", "b"]
//! doc.shares_storage_with(&snap) -> false
//! doc.set_line(9, "x") -> false
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` lines and `10^4` operations.
//!
//! ## Hints
//!
//! 1. `Rc::make_mut(&mut self.lines)` returns `&mut Vec<String>`. If the `Rc` is shared, it first
//!    clones the vector into a fresh allocation that only `self` owns; if not, it hands out the
//!    existing vector.
//! 2. Check the index **before** calling `make_mut`, so a failed `set_line` leaves sharing intact.

use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct Document {}

impl Document {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn from_lines(lines: &[&str]) -> Self {
        todo!("Write your solution here")
    }

    pub fn snapshot(&self) -> Document {
        todo!("Write your solution here")
    }

    pub fn lines(&self) -> &[String] {
        todo!("Write your solution here")
    }

    pub fn push_line(&mut self, line: &str) {
        todo!("Write your solution here")
    }

    pub fn set_line(&mut self, index: usize, line: &str) -> bool {
        todo!("Write your solution here")
    }

    pub fn shares_storage_with(&self, other: &Document) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writing_copies_shared_storage_once() {
        let mut doc = Document::from_lines(&["a", "b"]);
        let snap = doc.snapshot();
        assert!(doc.shares_storage_with(&snap));
        doc.push_line("c");
        assert_eq!(doc.lines(), ["a", "b", "c"]);
        assert_eq!(snap.lines(), ["a", "b"]);
        assert!(!doc.shares_storage_with(&snap));
    }

    #[test]
    fn unique_storage_is_edited_in_place() {
        let mut doc = Document::from_lines(&["x", "y"]);
        let buffer = doc.lines().as_ptr();
        assert!(doc.set_line(1, "z"));
        assert_eq!(doc.lines(), ["x", "z"]);
        // No copy happened: the same vector buffer is still in use.
        assert_eq!(doc.lines().as_ptr(), buffer);
    }

    #[test]
    fn failed_set_line_keeps_sharing() {
        let mut doc = Document::from_lines(&["only"]);
        let snap = doc.snapshot();
        assert!(!doc.set_line(1, "nope"));
        assert!(doc.shares_storage_with(&snap));
        assert_eq!(doc.lines(), ["only"]);
    }

    #[test]
    fn undo_history_of_snapshots() {
        let mut doc = Document::new();
        assert!(doc.lines().is_empty());
        let mut history = vec![doc.snapshot()];
        for word in ["one", "two", "three"] {
            doc.push_line(word);
            history.push(doc.snapshot());
        }
        assert!(doc.set_line(0, "ONE"));
        let lens: Vec<usize> = history.iter().map(|d| d.lines().len()).collect();
        assert_eq!(lens, vec![0, 1, 2, 3]);
        assert_eq!(history[3].lines(), ["one", "two", "three"]);
        assert_eq!(doc.lines(), ["ONE", "two", "three"]);
        assert!(history[2].shares_storage_with(&history[2].snapshot()));
    }
}
