// ==================================================================
// PRACTICE 0460: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0460_pagination_skip_take_chain.md
// Run the tests:          cargo test practice::p0460
// ==================================================================

//! # Problem 0460: Pagination with skip, take and chain
//!
//! - **Difficulty:** Easy
//! - **Topics:** `skip`, `take`, `chain`, `iter::once`, `enumerate` before `skip`, `usize::div_ceil`, `checked_*` arithmetic
//! - **Source:** Original · [std `Iterator::skip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip) · [std `Iterator::chain`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)
//!
//! ## Problem
//!
//! A web page shows a long list a few items at a time. Page `p` (pages are numbered from **1**)
//! with `per_page` items per page shows the items with indexes
//! `(p - 1) * per_page .. p * per_page`. Iterator adapters express this directly: skip the earlier
//! pages, then take one page.
//!
//! Implement:
//!
//! - `page<T: Clone>(items: &[T], page: usize, per_page: usize) -> Vec<T>`: the items of that page.
//!   Page `0`, `per_page == 0` or a page past the end give an empty vector.
//! - `page_count(total: usize, per_page: usize) -> usize`: the number of pages needed for `total`
//!   items (`0` when `per_page == 0` or `total == 0`).
//! - `numbered_page(items: &[&str], page: usize, per_page: usize) -> Vec<String>`: like `page`, but
//!   each item is prefixed with its **global** 1-based position: `"11. k"`.
//! - `framed(lines: &[&str], header: &str, footer: &str) -> Vec<String>`: the header, then the
//!   lines, then the footer.
//!
//! ## Examples
//!
//! ```text
//! items = ["a", "b", "c", "d", "e"]
//! page(items, 1, 2)             -> ["a", "b"]
//! page(items, 3, 2)             -> ["e"]
//! page(items, 4, 2)             -> []
//! page_count(5, 2)              -> 3
//! numbered_page(items, 2, 2)    -> ["3. c", "4. d"]
//! framed(&["x", "y"], "BEGIN", "END") -> ["BEGIN", "x", "y", "END"]
//! ```
//!
//! ## Constraints
//!
//! - `page` and `per_page` can be any `usize`; the functions must never panic (watch for overflow
//!   in `(page - 1) * per_page`).
//!
//! ## Hints
//!
//! 1. `items.iter().skip(start).take(per_page).cloned().collect()`.
//! 2. `page.checked_sub(1)` and `checked_mul` turn the dangerous arithmetic into `Option`s.
//! 3. Call `enumerate()` **before** `skip` so the indexes stay global; `iter::once(x)` is an
//!    iterator with a single item, handy with `chain`.

use std::iter;

pub fn page<T: Clone>(items: &[T], page: usize, per_page: usize) -> Vec<T> {
    todo!("Write your solution here")
}

pub fn page_count(total: usize, per_page: usize) -> usize {
    todo!("Write your solution here")
}

pub fn numbered_page(items: &[&str], page: usize, per_page: usize) -> Vec<String> {
    todo!("Write your solution here")
}

pub fn framed(lines: &[&str], header: &str, footer: &str) -> Vec<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEMS: [&str; 5] = ["a", "b", "c", "d", "e"];

    #[test]
    fn pages() {
        assert_eq!(page(&ITEMS, 1, 2), vec!["a", "b"]);
        assert_eq!(page(&ITEMS, 2, 2), vec!["c", "d"]);
        assert_eq!(page(&ITEMS, 3, 2), vec!["e"]);
        assert_eq!(page(&ITEMS, 4, 2), Vec::<&str>::new());
        assert_eq!(page(&[1, 2, 3], 1, 10), vec![1, 2, 3]);
    }

    #[test]
    fn invalid_pages_never_panic() {
        assert_eq!(page(&ITEMS, 0, 2), Vec::<&str>::new());
        assert_eq!(page(&ITEMS, 1, 0), Vec::<&str>::new());
        assert_eq!(page(&ITEMS, usize::MAX, usize::MAX), Vec::<&str>::new());
        assert_eq!(numbered_page(&ITEMS, usize::MAX, 2), Vec::<String>::new());
    }

    #[test]
    fn counting_pages() {
        assert_eq!(page_count(5, 2), 3);
        assert_eq!(page_count(4, 2), 2);
        assert_eq!(page_count(0, 2), 0);
        assert_eq!(page_count(5, 0), 0);
        assert_eq!(page_count(usize::MAX, 1), usize::MAX);
    }

    #[test]
    fn numbered_pages_use_global_positions() {
        assert_eq!(numbered_page(&ITEMS, 2, 2), vec!["3. c", "4. d"]);
        assert_eq!(numbered_page(&ITEMS, 1, 1), vec!["1. a"]);
        let many: Vec<String> = (0..30).map(|i| format!("item{i}")).collect();
        let refs: Vec<&str> = many.iter().map(String::as_str).collect();
        assert_eq!(
            numbered_page(&refs, 3, 5),
            vec![
                "11. item10",
                "12. item11",
                "13. item12",
                "14. item13",
                "15. item14"
            ]
        );
    }

    #[test]
    fn framed_lines() {
        assert_eq!(
            framed(&["x", "y"], "BEGIN", "END"),
            vec!["BEGIN", "x", "y", "END"]
        );
        assert_eq!(framed(&[], "<", ">"), vec!["<", ">"]);
    }
}
