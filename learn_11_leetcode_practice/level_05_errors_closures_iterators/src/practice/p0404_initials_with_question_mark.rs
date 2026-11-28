// ==================================================================
// PRACTICE 0404: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0404_initials_with_question_mark.md
// Run the tests:          cargo test practice::p0404
// ==================================================================

//! # Problem 0404: Initials with `?` on Option
//!
//! - **Difficulty:** Easy
//! - **Topics:** the `?` operator on `Option`, `split_whitespace`, `split_once`, `Result::ok`
//! - **Source:** Original · [The Rust Book 9.2: Where the `?` operator can be used](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used)
//!
//! ## Problem
//!
//! Inside a function that returns `Option<T>`, writing `expr?` means: "if `expr` is `None`,
//! return `None` from the whole function right now; otherwise give me the value inside `Some`".
//! It turns a ladder of nested `match` expressions into straight-line code.
//!
//! Implement three functions, each returning `None` as soon as a step fails:
//!
//! - `initials(full_name: &str) -> Option<String>`: the uppercase first letter of the **first**
//!   word and of the **last** word, each followed by a dot (`"ada lovelace"` -> `"A.L."`). Words
//!   are separated by any whitespace; middle names are ignored. A name with fewer than two words
//!   gives `None`.
//! - `email_domain(email: &str) -> Option<&str>`: the part after the `'@'`. Return `None` if there
//!   is no `'@'`, if the part before it is empty, or if the domain does not contain a `'.'`.
//!   Only the first `'@'` splits the address; a second `'@'` in the domain also gives `None`.
//! - `sum_first_two(text: &str) -> Option<i32>`: parse the first two whitespace-separated words
//!   as `i32` and return their sum. Return `None` if there are fewer than two words, if either
//!   word is not a valid `i32`, or if the sum overflows.
//!
//! ## Examples
//!
//! ```text
//! initials("ada lovelace")               -> Some("A.L.")
//! initials("  grace  brewster  hopper ") -> Some("G.H.")
//! initials("plato")                      -> None
//! email_domain("ann@example.com")        -> Some("example.com")
//! email_domain("@example.com")           -> None   (empty name part)
//! email_domain("ann@localhost")          -> None   (no dot in the domain)
//! sum_first_two("12 30 and more")        -> Some(42)
//! sum_first_two("12 apples")             -> None
//! ```
//!
//! ## Constraints
//!
//! - Inputs are arbitrary strings (possibly empty). Initials only need to handle ASCII letters
//!   correctly, but must not panic on other characters.
//!
//! ## Hints
//!
//! 1. `let mut words = full_name.split_whitespace();` then `words.next()?` takes the first word,
//!    and after that `words.last()?` takes the last remaining one.
//! 2. `s.split_once('@')?` gives you the two halves as a tuple.
//! 3. `s.parse::<i32>()` returns a `Result`; `.ok()?` converts it to an `Option` and applies `?`.

pub fn initials(full_name: &str) -> Option<String> {
    todo!("Write your solution here")
}

pub fn email_domain(email: &str) -> Option<&str> {
    todo!("Write your solution here")
}

pub fn sum_first_two(text: &str) -> Option<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_of_first_and_last_word() {
        assert_eq!(initials("ada lovelace").as_deref(), Some("A.L."));
        assert_eq!(
            initials("  grace  brewster  hopper ").as_deref(),
            Some("G.H.")
        );
        assert_eq!(initials("Alan\tTuring").as_deref(), Some("A.T."));
    }

    #[test]
    fn initials_need_two_words() {
        assert_eq!(initials("plato"), None);
        assert_eq!(initials(""), None);
        assert_eq!(initials("    "), None);
    }

    #[test]
    fn email_domain_valid_addresses() {
        assert_eq!(email_domain("ann@example.com"), Some("example.com"));
        assert_eq!(email_domain("a.b@mail.co.uk"), Some("mail.co.uk"));
    }

    #[test]
    fn email_domain_invalid_addresses() {
        assert_eq!(email_domain("@example.com"), None);
        assert_eq!(email_domain("ann@localhost"), None);
        assert_eq!(email_domain("no-at-sign.com"), None);
        assert_eq!(email_domain("a@b@c.com"), None);
    }

    #[test]
    fn sum_first_two_parses_and_adds() {
        assert_eq!(sum_first_two("12 30 and more"), Some(42));
        assert_eq!(sum_first_two("-5 5"), Some(0));
        assert_eq!(sum_first_two("12 apples"), None);
        assert_eq!(sum_first_two("12"), None);
        assert_eq!(sum_first_two("2147483647 1"), None);
    }
}
