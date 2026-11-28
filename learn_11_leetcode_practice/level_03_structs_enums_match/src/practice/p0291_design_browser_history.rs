// ==================================================================
// PRACTICE 0291: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0291_design_browser_history.md
// Run the tests:          cargo test practice::p0291
// ==================================================================

//! # Problem 0291: Design Browser History
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct with a `Vec` and a cursor, `Vec::truncate`, clamping with `min` / `saturating_sub`, `&mut self` methods
//! - **Source:** [LeetCode 1472. Design Browser History](https://leetcode.com/problems/design-browser-history/)
//!
//! ## Problem
//!
//! A browser tab starts on a `homepage`. Implement its navigation history as `BrowserHistory`:
//!
//! - `BrowserHistory::new(homepage: String) -> Self`;
//! - `visit(&mut self, url: String)`: go to `url` from the current page. This **erases all forward
//!   history** (the pages you could have reached with `forward`);
//! - `back(&mut self, steps: i32) -> String`: move back up to `steps` pages (stop at the first page
//!   if there are fewer) and return the current URL;
//! - `forward(&mut self, steps: i32) -> String`: move forward up to `steps` pages (stop at the last
//!   page) and return the current URL.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`. LeetCode's template
//! declares `&self` on every method and suggests `&mut self` where needed, as done here.
//!
//! ## Examples
//!
//! ```text
//! let mut h = BrowserHistory::new("leetcode.com".into());
//! h.visit("google.com".into());
//! h.visit("facebook.com".into());
//! h.visit("youtube.com".into());
//! h.back(1)        => "facebook.com"
//! h.back(1)        => "google.com"
//! h.forward(1)     => "facebook.com"
//! h.visit("linkedin.com".into());     // "youtube.com" is no longer reachable
//! h.forward(2)     => "linkedin.com"
//! h.back(2)        => "google.com"
//! h.back(7)        => "leetcode.com"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= steps <= 100`; at most `5000` calls in total.
//!
//! ## Hints
//!
//! 1. Store the pages in a `Vec<String>` and the current position in a `usize`.
//! 2. `visit` = `truncate(current + 1)`, then `push`.

/// The navigation history of one browser tab.
pub struct BrowserHistory {}

impl BrowserHistory {
    /// A history containing only `homepage`.
    pub fn new(homepage: String) -> Self {
        todo!("Write your solution here")
    }

    /// Goes to `url`, erasing the forward history.
    pub fn visit(&mut self, url: String) {
        todo!("Write your solution here")
    }

    /// Moves back up to `steps` pages.
    pub fn back(&mut self, steps: i32) -> String {
        todo!("Write your solution here")
    }

    /// Moves forward up to `steps` pages.
    pub fn forward(&mut self, steps: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut h = BrowserHistory::new("leetcode.com".into());
        h.visit("google.com".into());
        h.visit("facebook.com".into());
        h.visit("youtube.com".into());
        assert_eq!(h.back(1), "facebook.com");
        assert_eq!(h.back(1), "google.com");
        assert_eq!(h.forward(1), "facebook.com");
        h.visit("linkedin.com".into());
        assert_eq!(h.forward(2), "linkedin.com");
        assert_eq!(h.back(2), "google.com");
        assert_eq!(h.back(7), "leetcode.com");
    }

    #[test]
    fn only_homepage() {
        let mut h = BrowserHistory::new("home".into());
        assert_eq!(h.back(5), "home");
        assert_eq!(h.forward(5), "home");
    }

    #[test]
    fn visit_after_back_erases_forward_pages() {
        let mut h = BrowserHistory::new("a".into());
        h.visit("b".into());
        h.visit("c".into());
        assert_eq!(h.back(2), "a");
        h.visit("d".into());
        assert_eq!(h.forward(1), "d");
        assert_eq!(h.back(1), "a");
        assert_eq!(h.forward(10), "d");
    }

    #[test]
    fn many_visits() {
        let mut h = BrowserHistory::new("p0".into());
        for i in 1..4_990 {
            h.visit(format!("p{i}"));
        }
        assert_eq!(h.back(100), "p4889");
        assert_eq!(h.forward(50), "p4939");
        assert_eq!(h.forward(100), "p4989");
    }
}
