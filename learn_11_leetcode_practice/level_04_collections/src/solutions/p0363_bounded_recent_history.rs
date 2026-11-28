//! # Problem 0363: Bounded Recent History
//!
//! - **Difficulty:** Easy
//! - **Topics:** `VecDeque::push_front`, `pop_back`, `pop_front`, `remove`, `iter().position`, `truncate`
//! - **Source:** Original · [`VecDeque::push_front`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.push_front)
//!
//! ## Problem
//!
//! A browser keeps a list of recently visited pages, most recent first, with a fixed capacity.
//! Implement `History`:
//!
//! - `History::new(capacity: usize) -> Self`: an empty history that keeps at most `capacity` pages
//!   (`0` means nothing is ever kept);
//! - `visit(&mut self, page: &str)`: `page` becomes the most recent entry. If it was already in the
//!   history, its old entry is removed first (no duplicates). If the history now holds more than
//!   `capacity` pages, the **oldest** ones are dropped;
//! - `recent(&self, n: usize) -> Vec<String>`: the `n` most recent pages, most recent first (all of
//!   them if there are fewer);
//! - `go_back(&mut self) -> Option<String>`: remove and return the most recent page;
//! - `len(&self) -> usize` and `is_empty(&self) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! let mut h = History::new(3);
//! h.visit("a"); h.visit("b"); h.visit("c");
//! h.recent(10)   => ["c", "b", "a"]
//! h.visit("d");                       (capacity 3: "a" is dropped)
//! h.recent(10)   => ["d", "c", "b"]
//! h.visit("b");                       ("b" moves to the front)
//! h.recent(2)    => ["b", "d"]
//! h.go_back()    => Some("b")
//! h.recent(10)   => ["d", "c"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= capacity <= 1000`, at most `10^4` operations.
//!
//! ## Hints
//!
//! 1. Keep the most recent page at the **front** of a `VecDeque` (`push_front`), so the oldest one
//!    is at the back (`pop_back` or `truncate`).
//! 2. `iter().position(|p| p == page)` finds an existing entry; `remove(i)` deletes it and returns
//!    `Option<String>`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a `VecDeque<String>` ordered from newest (front) to oldest (back). `visit` removes a
//! previous entry of the same page, pushes the page at the front, and `truncate(capacity)` cuts the
//! oldest entries. `recent` reads from the front; `go_back` pops the front.
//!
//! **Rust concepts:**
//! - `push_front` / `pop_front` / `pop_back` are `O(1)` on a `VecDeque`.
//! - `VecDeque::remove(i)` returns `Option<T>` (`None` if `i` is out of bounds), unlike
//!   `Vec::remove`, which panics.
//! - `truncate(n)` keeps the first `n` elements, i.e. drops from the back.
//! - Providing `is_empty` next to `len` is what Clippy's `len_without_is_empty` lint expects.
// @solution-end

use std::collections::VecDeque;

/// Recently visited pages, most recent first, with a capacity.
#[derive(Debug)]
pub struct History {
    // @solution-start remove
    pages: VecDeque<String>,
    capacity: usize,
    // @solution-end
}

impl History {
    /// Creates an empty history holding at most `capacity` pages.
    pub fn new(capacity: usize) -> Self {
        // @solution-start
        Self {
            pages: VecDeque::with_capacity(capacity.min(1024)),
            capacity,
        }
        // @solution-end
    }

    /// Records a visit to `page`.
    pub fn visit(&mut self, page: &str) {
        // @solution-start
        if let Some(i) = self.pages.iter().position(|p| p == page) {
            self.pages.remove(i);
        }
        self.pages.push_front(page.to_string());
        self.pages.truncate(self.capacity);
        // @solution-end
    }

    /// The `n` most recent pages, most recent first.
    pub fn recent(&self, n: usize) -> Vec<String> {
        // @solution-start
        self.pages.iter().take(n).cloned().collect()
        // @solution-end
    }

    /// Removes and returns the most recent page.
    pub fn go_back(&mut self) -> Option<String> {
        // @solution-start
        self.pages.pop_front()
        // @solution-end
    }

    /// Number of stored pages.
    pub fn len(&self) -> usize {
        // @solution-start
        self.pages.len()
        // @solution-end
    }

    /// Whether no page is stored.
    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.pages.is_empty()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut h = History::new(3);
        h.visit("a");
        h.visit("b");
        h.visit("c");
        assert_eq!(h.recent(10), vec!["c", "b", "a"]);
        h.visit("d");
        assert_eq!(h.recent(10), vec!["d", "c", "b"]);
        h.visit("b");
        assert_eq!(h.recent(2), vec!["b", "d"]);
        assert_eq!(h.go_back(), Some("b".to_string()));
        assert_eq!(h.recent(10), vec!["d", "c"]);
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn capacity_zero_keeps_nothing() {
        let mut h = History::new(0);
        h.visit("x");
        assert!(h.is_empty());
        assert_eq!(h.go_back(), None);
    }

    #[test]
    fn revisiting_the_newest_page_changes_nothing() {
        let mut h = History::new(2);
        h.visit("a");
        h.visit("a");
        assert_eq!(h.len(), 1);
        assert_eq!(h.recent(0), Vec::<String>::new());
        assert!(!h.is_empty());
    }

    #[test]
    fn many_visits() {
        let mut h = History::new(1000);
        for i in 0..10_000 {
            h.visit(&format!("p{}", i % 1500));
        }
        assert_eq!(h.len(), 1000);
        assert_eq!(h.recent(1), vec!["p999"]);
        // The oldest kept page was visited 999 steps before the last one.
        assert_eq!(h.recent(1000)[999], "p0");
    }
}
