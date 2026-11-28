// ==================================================================
// PRACTICE 0298: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0298_design_a_text_editor.md
// Run the tests:          cargo test practice::p0298
// ==================================================================

//! # Problem 0298: Design a Text Editor
//!
//! - **Difficulty:** Hard
//! - **Topics:** design struct with two stacks around a cursor, moving characters between `Vec`s, `split_off`, returning a window of text
//! - **Source:** [LeetCode 2296. Design a Text Editor](https://leetcode.com/problems/design-a-text-editor/)
//!
//! ## Problem
//!
//! Build a text editor with a cursor. The cursor always sits between two characters (or at an end
//! of the text), and can never leave the text.
//!
//! - `TextEditor::new() -> Self`: empty text, cursor at position `0`;
//! - `add_text(&mut self, text: String)`: insert `text` at the cursor; the cursor ends up **after**
//!   the inserted text;
//! - `delete_text(&mut self, k: i32) -> i32`: delete up to `k` characters to the **left** of the
//!   cursor (like pressing Backspace `k` times); return how many were actually deleted;
//! - `cursor_left(&mut self, k: i32) -> String` and `cursor_right(&mut self, k: i32) -> String`:
//!   move the cursor `k` positions (stopping at the ends of the text), then return the last
//!   `min(10, len)` characters to the **left** of the cursor, where `len` is the number of
//!   characters left of the cursor.
//!
//! Every operation must cost time proportional to `k` (or to the inserted text), not to the length
//! of the whole document. This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut e = TextEditor::new();
//! e.add_text("leetcode".into());   // "leetcode|"
//! e.delete_text(4)    => 4         // "leet|"
//! e.add_text("practice".into());   // "leetpractice|"
//! e.cursor_right(3)   => "etpractice"   (already at the end; last 10 chars on the left)
//! e.cursor_left(8)    => "leet"         // "leet|practice"
//! e.delete_text(10)   => 4         // "|practice"
//! e.cursor_left(2)    => ""
//! e.cursor_right(6)   => "practi"       // "practi|ce"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= text.len(), k <= 40`; `text` is lowercase English letters; at most `2 * 10^4` calls.
//!
//! ## Hints
//!
//! 1. Store the text left of the cursor in `left: Vec<u8>` (in order) and the text right of the
//!    cursor in `right: Vec<u8>` **reversed**, so that both sides grow and shrink at the end.
//! 2. Moving the cursor left by one is `right.push(left.pop())`.

/// A text editor with a cursor.
pub struct TextEditor {}

impl TextEditor {
    /// An empty editor.
    // LeetCode's template has an argument-less `new` and no `Default`.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Inserts `text` at the cursor.
    pub fn add_text(&mut self, text: String) {
        todo!("Write your solution here")
    }

    /// Deletes up to `k` characters left of the cursor; returns how many.
    pub fn delete_text(&mut self, k: i32) -> i32 {
        todo!("Write your solution here")
    }

    /// Moves the cursor left; returns up to 10 characters left of it.
    pub fn cursor_left(&mut self, k: i32) -> String {
        todo!("Write your solution here")
    }

    /// Moves the cursor right; returns up to 10 characters left of it.
    pub fn cursor_right(&mut self, k: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut e = TextEditor::new();
        e.add_text("leetcode".into());
        assert_eq!(e.delete_text(4), 4);
        e.add_text("practice".into());
        assert_eq!(e.cursor_right(3), "etpractice");
        assert_eq!(e.cursor_left(8), "leet");
        assert_eq!(e.delete_text(10), 4);
        assert_eq!(e.cursor_left(2), "");
        assert_eq!(e.cursor_right(6), "practi");
    }

    #[test]
    fn insert_in_the_middle() {
        let mut e = TextEditor::new();
        e.add_text("helloworld".into());
        assert_eq!(e.cursor_left(5), "hello");
        e.add_text("_".into());
        // "hello_world" has 11 characters; the last 10 are shown.
        assert_eq!(e.cursor_right(40), "ello_world");
        assert_eq!(e.cursor_left(40), "");
        assert_eq!(e.cursor_right(1), "h");
    }

    #[test]
    fn delete_more_than_available() {
        let mut e = TextEditor::new();
        assert_eq!(e.delete_text(5), 0);
        e.add_text("ab".into());
        assert_eq!(e.cursor_left(1), "a");
        assert_eq!(e.delete_text(5), 1);
        assert_eq!(e.cursor_right(1), "b");
    }

    #[test]
    fn many_operations() {
        let mut e = TextEditor::new();
        for _ in 0..5_000 {
            e.add_text("abcd".into());
        }
        assert_eq!(e.cursor_left(2), "abcdabcdab");
        assert_eq!(e.delete_text(40), 40);
        // Only "cd" was right of the cursor.
        assert_eq!(e.cursor_right(40), "cdabcdabcd");
        assert_eq!(e.cursor_left(10).len(), 10);
    }
}
