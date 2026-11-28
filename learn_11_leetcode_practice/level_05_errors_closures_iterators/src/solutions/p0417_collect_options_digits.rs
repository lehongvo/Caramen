//! # Problem 0417: All Digits or Nothing
//!
//! - **Difficulty:** Easy
//! - **Topics:** `collect::<Option<Vec<_>>>()`, `Sum` for `Option`, `char::to_digit`, `bool::then`, `?` on `Option`
//! - **Source:** Original · [std `Option`: collecting into `Option`](https://doc.rust-lang.org/std/option/index.html#collecting-into-option)
//!
//! ## Problem
//!
//! The "all or nothing" trick from `Result` works for `Option` too: an iterator of `Option<T>` can
//! be collected into an `Option<Vec<T>>`, which is `None` as soon as one item is `None`.
//!
//! Implement:
//!
//! - `digits(s: &str) -> Option<Vec<u32>>`: the decimal digits of `s` as numbers, or `None` if any
//!   character is not an ASCII digit `'0'..='9'`. An empty string gives `Some(vec![])`.
//! - `digit_sum(s: &str) -> Option<u32>`: the sum of those digits, or `None` (empty string -> `Some(0)`).
//! - `hex_value(s: &str) -> Option<u64>`: the value of `s` read as a hexadecimal number (digits
//!   `0-9`, `a-f`, `A-F`). Return `None` if `s` is empty, longer than 16 characters, or contains a
//!   non-hex character. (16 hex digits always fit in a `u64`.)
//! - `word_lengths(words: &[&str]) -> Option<Vec<usize>>`: the number of characters of every word,
//!   but `None` if some word contains a character that is not alphabetic (`char::is_alphabetic`).
//!
//! ## Examples
//!
//! ```text
//! digits("2024")                 -> Some([2, 0, 2, 4])
//! digits("20x4")                 -> None
//! digit_sum("999")               -> Some(27)
//! hex_value("ff")                -> Some(255)
//! hex_value("DEADBEEF")          -> Some(3735928559)
//! hex_value("0x1f")              -> None          ('x' is not a hex digit)
//! word_lengths(&["über", "ab"])  -> Some([4, 2])  (characters, not bytes)
//! word_lengths(&["ab", "a1"])    -> None
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `c.to_digit(10)` returns `Option<u32>`; `to_digit(16)` accepts both upper- and lower-case
//!    hex letters.
//! 2. `s.chars().map(|c| c.to_digit(10)).collect()` and `.sum()` both work on `Option`s.
//! 3. `condition.then(|| value)` returns `Some(value)` when `condition` is `true`, else `None`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** turn every character (or word) into an `Option` and let `collect` / `sum` combine
//! them. `hex_value` collects the digit values first (with `?` to leave early on a bad character)
//! and then folds them into a number, `acc * 16 + digit`.
//!
//! **Rust concepts:**
//! - `Option<V>` implements `FromIterator<Option<T>>` when `V: FromIterator<T>`, and
//!   `Option<T>: Sum<Option<T>>` when `T: Sum`.
//! - Both stop at the first `None`.
//! - `bool::then(closure)` builds an `Option` from a condition; `then_some(value)` is the eager form.
//! - `chars().count()` counts Unicode scalar values; `len()` would count UTF-8 bytes (`"ü"` is 2).
// @solution-end

pub fn digits(s: &str) -> Option<Vec<u32>> {
    // @solution-start
    s.chars().map(|c| c.to_digit(10)).collect()
    // @solution-end
}

pub fn digit_sum(s: &str) -> Option<u32> {
    // @solution-start
    s.chars().map(|c| c.to_digit(10)).sum()
    // @solution-end
}

pub fn hex_value(s: &str) -> Option<u64> {
    // @solution-start
    if s.is_empty() || s.chars().count() > 16 {
        return None;
    }
    let values: Vec<u64> = s
        .chars()
        .map(|c| c.to_digit(16).map(u64::from))
        .collect::<Option<Vec<u64>>>()?;
    Some(values.into_iter().fold(0, |acc, d| acc * 16 + d))
    // @solution-end
}

pub fn word_lengths(words: &[&str]) -> Option<Vec<usize>> {
    // @solution-start
    words
        .iter()
        .map(|w| {
            w.chars()
                .all(char::is_alphabetic)
                .then(|| w.chars().count())
        })
        .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_all_or_nothing() {
        assert_eq!(digits("2024"), Some(vec![2, 0, 2, 4]));
        assert_eq!(digits("20x4"), None);
        assert_eq!(digits(""), Some(vec![]));
        assert_eq!(digits("-1"), None);
    }

    #[test]
    fn digit_sum_cases() {
        assert_eq!(digit_sum("999"), Some(27));
        assert_eq!(digit_sum(""), Some(0));
        assert_eq!(digit_sum("9 9"), None);
    }

    #[test]
    fn hex_values() {
        assert_eq!(hex_value("ff"), Some(255));
        assert_eq!(hex_value("DEADBEEF"), Some(3_735_928_559));
        assert_eq!(hex_value("0"), Some(0));
        assert_eq!(hex_value("ffffffffffffffff"), Some(u64::MAX));
    }

    #[test]
    fn hex_rejections() {
        assert_eq!(hex_value("0x1f"), None);
        assert_eq!(hex_value(""), None);
        assert_eq!(hex_value("10000000000000000"), None);
        assert_eq!(hex_value("+f"), None);
    }

    #[test]
    fn word_lengths_count_characters() {
        assert_eq!(word_lengths(&["über", "ab"]), Some(vec![4, 2]));
        assert_eq!(word_lengths(&["ab", "a1"]), None);
        assert_eq!(word_lengths(&[]), Some(vec![]));
        assert_eq!(word_lengths(&[""]), Some(vec![0]));
    }
}
