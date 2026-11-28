// ==================================================================
// PRACTICE 0923: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0923_sealed_extension_trait.md
// Run the tests:          cargo test practice::p0923
// ==================================================================

//! # Problem 0923: A Sealed Extension Trait
//!
//! - **Difficulty:** Medium
//! - **Topics:** extension traits, implementing a trait for `str` (an unsized type), sealing, `?Sized` bounds, delegating impls
//! - **Source:** Original · [Reference: Implementations](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations) · [The Rust Book 10.2: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
//!
//! ## Problem
//!
//! You cannot add inherent methods to `str`, but you can add methods through an **extension trait**.
//! Sealing it (only this module may implement it) lets you add more methods in a later version
//! without breaking anyone, because nobody else can have an `impl TextExt for ...` that would lack
//! the new methods.
//!
//! ```text
//! mod sealed { pub trait Sealed {} }      // implemented for str and String only
//! pub trait TextExt: sealed::Sealed { ... }
//! ```
//!
//! Implement `TextExt for str` and, by **delegating** to it, `TextExt for String`:
//!
//! - `word_count(&self) -> usize`: the number of whitespace-separated words.
//! - `is_palindrome(&self) -> bool`: ignoring case and every character that is not alphanumeric
//!   (`"A man, a plan, a canal: Panama"` is one). An empty or symbol-only text is a palindrome.
//! - `truncate_words(&self, max_words: usize) -> String`: the first `max_words` words joined by single
//!   spaces, followed by `"..."` if any word was dropped.
//! - `title_case(&self) -> String`: every word's first character uppercased and the rest lowercased,
//!   words joined by single spaces.
//!
//! Then write `count_palindromes<S: TextExt + ?Sized>(items: &[&S]) -> usize`, which works for
//! slices of `&str` and of `&String`.
//!
//! ## Examples
//!
//! ```text
//! "  hello   big world ".word_count()             => 3
//! "Was it a car or a cat I saw?".is_palindrome()   => true
//! "one two three four".truncate_words(2)           => "one two..."
//! String::from("rUST is FUN").title_case()         => "Rust Is Fun"
//! count_palindromes(&["noon", "rust", "Level"])    => 2
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Traits have an implicit `Self: ?Sized`, so `impl TextExt for str` is allowed; generic
//!    functions need an explicit `?Sized` to accept `S = str`.
//! 2. For `String`, each method can call `self.as_str().method(...)`.
//! 3. Compare the filtered lowercase characters with their reverse using a double-ended iterator.

mod sealed {
    pub trait Sealed {}
    impl Sealed for str {}
    impl Sealed for String {}
}

pub trait TextExt: sealed::Sealed {
    fn word_count(&self) -> usize;
    fn is_palindrome(&self) -> bool;
    fn truncate_words(&self, max_words: usize) -> String;
    fn title_case(&self) -> String;
}

impl TextExt for str {
    fn word_count(&self) -> usize {
        todo!("Write your solution here")
    }

    fn is_palindrome(&self) -> bool {
        todo!("Write your solution here")
    }

    fn truncate_words(&self, max_words: usize) -> String {
        todo!("Write your solution here")
    }

    fn title_case(&self) -> String {
        todo!("Write your solution here")
    }
}

impl TextExt for String {
    fn word_count(&self) -> usize {
        todo!("Write your solution here")
    }

    fn is_palindrome(&self) -> bool {
        todo!("Write your solution here")
    }

    fn truncate_words(&self, max_words: usize) -> String {
        todo!("Write your solution here")
    }

    fn title_case(&self) -> String {
        todo!("Write your solution here")
    }
}

pub fn count_palindromes<S: TextExt + ?Sized>(items: &[&S]) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_counts() {
        assert_eq!("  hello   big world ".word_count(), 3);
        assert_eq!("".word_count(), 0);
        assert_eq!(String::from("a\tb\nc").word_count(), 3);
    }

    #[test]
    fn palindromes() {
        assert!("Was it a car or a cat I saw?".is_palindrome());
        assert!("A man, a plan, a canal: Panama".is_palindrome());
        assert!(!"rust".is_palindrome());
        assert!("".is_palindrome());
        assert!("?!".is_palindrome());
        assert!(String::from("No 'x' in Nixon").is_palindrome());
    }

    #[test]
    fn truncation_and_title_case() {
        assert_eq!("one two three four".truncate_words(2), "one two...");
        assert_eq!("one   two".truncate_words(2), "one two");
        assert_eq!("one".truncate_words(0), "...");
        assert_eq!("".truncate_words(0), "");
        assert_eq!(String::from("rUST is FUN").title_case(), "Rust Is Fun");
        assert_eq!(
            "  stra\u{df}e  \u{e9}t\u{e9} ".title_case(),
            "Stra\u{df}e \u{c9}t\u{e9}"
        );
    }

    #[test]
    fn generic_over_str_and_string() {
        assert_eq!(count_palindromes::<str>(&["noon", "rust", "Level"]), 2);
        let owned = [String::from("abba"), String::from("abc")];
        let refs: Vec<&String> = owned.iter().collect();
        assert_eq!(count_palindromes(&refs), 1);
        let long = "ab".repeat(50_000) + "a";
        assert!(long.is_palindrome());
    }
}
