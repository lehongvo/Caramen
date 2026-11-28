// ==================================================================
// PRACTICE 0571: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0571_two_lifetime_matcher.md
// Run the tests:          cargo test practice::p0571
// ==================================================================

//! # Problem 0571: A Matcher with Two Lifetimes
//!
//! - **Difficulty:** Medium
//! - **Topics:** a struct with lifetime `'p` and methods with their own lifetime `'t`, results tied to an argument instead of `self`
//! - **Source:** Original · [The Rust Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-method-definitions)
//!
//! ## Problem
//!
//! `Matcher<'p>` stores a borrowed, non-empty pattern (`pattern: &'p str`). Its search methods take a
//! text with a **different** lifetime `'t` and return slices of the **text**:
//!
//! - `Matcher::new(pattern: &'p str) -> Self`: panics with `"pattern must not be empty"` for `""`.
//! - `pattern(&self) -> &'p str`.
//! - `count_in(&self, text: &str) -> usize`: the number of non-overlapping occurrences.
//! - `words_containing<'t>(&self, text: &'t str) -> Vec<&'t str>`: the whitespace-separated words of
//!   `text` that contain the pattern, in order.
//! - `split_around<'t>(&self, text: &'t str) -> Option<(&'t str, &'t str)>`: the text before and after
//!   the **first** occurrence (the pattern itself excluded); `None` if it does not occur.
//!
//! The results of `words_containing` and `split_around` must remain usable after the `Matcher` and
//! its pattern are gone. With elision (`fn words_containing(&self, text: &str) -> Vec<&str>`), rule 3
//! would tie the results to `&self`: the body would not compile (it returns slices of `text`), and even
//! if it did, the results could not outlive the `Matcher`. Name `'t` explicitly.
//!
//! ## Examples
//!
//! ```text
//! let m = Matcher::new("an");
//! m.count_in("banana")                          => 2
//! m.words_containing("a banana and a mango")    => ["banana", "and", "mango"]
//! m.split_around("pandas eat bamboo")           => Some(("p", "das eat bamboo"))
//! m.split_around("xyz")                         => None
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `text.matches(self.pattern).count()` counts non-overlapping matches.
//! 2. `text.find(self.pattern)` gives the byte index of the first match; slice before it and after
//!    `index + pattern.len()`.

/// Searches texts for a borrowed pattern.
#[derive(Debug, Clone, Copy)]
pub struct Matcher<'p> {
    pattern: &'p str,
}

impl<'p> Matcher<'p> {
    pub fn new(pattern: &'p str) -> Self {
        todo!("Write your solution here")
    }

    pub fn pattern(&self) -> &'p str {
        todo!("Write your solution here")
    }

    pub fn count_in(&self, text: &str) -> usize {
        todo!("Write your solution here")
    }

    pub fn words_containing<'t>(&self, text: &'t str) -> Vec<&'t str> {
        todo!("Write your solution here")
    }

    pub fn split_around<'t>(&self, text: &'t str) -> Option<(&'t str, &'t str)> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_and_words() {
        let m = Matcher::new("an");
        assert_eq!(m.pattern(), "an");
        assert_eq!(m.count_in("banana"), 2);
        assert_eq!(m.count_in("aaaa"), 0);
        assert_eq!(
            m.words_containing("a banana and a mango"),
            vec!["banana", "and", "mango"]
        );
        assert_eq!(Matcher::new("aa").count_in("aaaa"), 2);
    }

    #[test]
    fn splitting_around_the_first_match() {
        let m = Matcher::new("an");
        assert_eq!(
            m.split_around("pandas eat bamboo"),
            Some(("p", "das eat bamboo"))
        );
        assert_eq!(m.split_around("an"), Some(("", "")));
        assert_eq!(m.split_around("xyz"), None);
        let euro = Matcher::new("\u{20ac}");
        assert_eq!(euro.split_around("5\u{20ac} each"), Some(("5", " each")));
    }

    #[test]
    fn results_outlive_the_pattern_and_the_matcher() {
        let text = String::from("cat catalog dog category");
        let found;
        let parts;
        {
            let pattern = String::from("cat");
            let m = Matcher::new(&pattern);
            found = m.words_containing(&text);
            parts = m.split_around(&text);
        } // `pattern` and `m` are gone; the results borrow only from `text`
        assert_eq!(found, vec!["cat", "catalog", "category"]);
        assert_eq!(parts, Some(("", " catalog dog category")));
    }

    #[test]
    #[should_panic(expected = "pattern must not be empty")]
    fn empty_pattern_panics() {
        let _ = Matcher::new("");
    }
}
