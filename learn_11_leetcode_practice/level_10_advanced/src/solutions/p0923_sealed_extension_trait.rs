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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** all logic lives in the `str` impl, built from `split_whitespace`, `chars().rev()` and
//! `to_uppercase`/`to_lowercase`; the `String` impl forwards every call to `as_str()`. The palindrome
//! check collects the normalised characters once and compares the vector with its reverse.
//!
//! **Complexity:** every method is linear in the length of the text.
//!
//! **Rust concepts:**
//! - An extension trait is the idiomatic way to add methods to foreign types such as `str`; users
//!   must `use` the trait to call them.
//! - The sealed supertrait keeps the set of implementors closed, which is why adding a method to
//!   `TextExt` later would not be a breaking change.
//! - `S: TextExt + ?Sized` lets the generic function accept the unsized `str` behind a reference.
// @solution-end

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
        // @solution-start
        self.split_whitespace().count()
        // @solution-end
    }

    fn is_palindrome(&self) -> bool {
        // @solution-start
        let normalized: Vec<char> = self
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect();
        normalized.iter().eq(normalized.iter().rev())
        // @solution-end
    }

    fn truncate_words(&self, max_words: usize) -> String {
        // @solution-start
        let words: Vec<&str> = self.split_whitespace().collect();
        let kept = words[..words.len().min(max_words)].join(" ");
        if words.len() > max_words {
            format!("{kept}...")
        } else {
            kept
        }
        // @solution-end
    }

    fn title_case(&self) -> String {
        // @solution-start
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first
                        .to_uppercase()
                        .chain(chars.flat_map(char::to_lowercase))
                        .collect(),
                    None => String::new(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
        // @solution-end
    }
}

impl TextExt for String {
    fn word_count(&self) -> usize {
        // @solution-start
        self.as_str().word_count()
        // @solution-end
    }

    fn is_palindrome(&self) -> bool {
        // @solution-start
        self.as_str().is_palindrome()
        // @solution-end
    }

    fn truncate_words(&self, max_words: usize) -> String {
        // @solution-start
        self.as_str().truncate_words(max_words)
        // @solution-end
    }

    fn title_case(&self) -> String {
        // @solution-start
        self.as_str().title_case()
        // @solution-end
    }
}

pub fn count_palindromes<S: TextExt + ?Sized>(items: &[&S]) -> usize {
    // @solution-start
    items.iter().filter(|item| item.is_palindrome()).count()
    // @solution-end
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
