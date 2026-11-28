// ==================================================================
// PRACTICE 0475: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0475_run_length_encoding_peekable.md
// Run the tests:          cargo test practice::p0475
// ==================================================================

//! # Problem 0475: Run-Length Encoding with peekable
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Iterator::peekable`, `Peekable::next_if_eq`, `Peekable::next_if`, `while let`, decoding with errors
//! - **Source:** Original · [std `Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html) · [std `Peekable::next_if`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if)
//!
//! ## Problem
//!
//! *Run-length encoding* replaces each run of equal characters by its length and the character:
//! `"aaabccdddd"` becomes `"3a1b2c4d"`. Grouping a run needs to look at the next character
//! **without consuming it** when it is different. `peekable()` adds exactly that ability to any
//! iterator: `peek()` looks ahead, and `next_if_eq(&x)` / `next_if(pred)` consume the next item
//! only when it matches.
//!
//! Implement:
//!
//! - `encode(s: &str) -> String`. The input never contains ASCII digits.
//! - `decode(s: &str) -> Result<String, String>`: the inverse. A count is one or more digits and is
//!   followed by the character to repeat. Errors (first problem wins):
//!   - a character without a count before it -> ``"missing count before `C`"``;
//!   - a count of zero -> `"zero count"`;
//!   - digits at the end -> ``"missing character after `N`"`` (`N` = the digits as written);
//!   - a count that does not fit in a `usize` -> `"count too large"`.
//! - `runs(nums: &[i32]) -> Vec<(i32, usize)>`: the runs of a slice as `(value, length)` pairs.
//!
//! ## Examples
//!
//! ```text
//! encode("aaabccdddd")   -> "3a1b2c4d"
//! encode("")             -> ""
//! decode("3a1b2c4d")     -> Ok("aaabccdddd")
//! decode("12x")          -> Ok("xxxxxxxxxxxx")
//! decode("a")            -> Err("missing count before `a`")
//! decode("0a")           -> Err("zero count")
//! decode("3a12")         -> Err("missing character after `12`")
//! runs(&[5, 5, 1, 5, 5, 5]) -> [(5, 2), (1, 1), (5, 3)]
//! ```
//!
//! ## Constraints
//!
//! - Inputs have at most `10^4` characters; decoded outputs stay below `10^6` characters in the tests.
//!
//! ## Hints
//!
//! 1. `let mut it = s.chars().peekable(); while let Some(c) = it.next() { ... }` handles one run per
//!    loop iteration; inside, `while it.next_if_eq(&c).is_some() { count += 1; }`.
//! 2. To read a number, collect digits with `next_if(|c| c.is_ascii_digit())` in a loop.
//! 3. `str::repeat` or `iter::repeat(c).take(n)` produces the decoded run.

pub fn encode(s: &str) -> String {
    todo!("Write your solution here")
}

pub fn decode(s: &str) -> Result<String, String> {
    todo!("Write your solution here")
}

pub fn runs(nums: &[i32]) -> Vec<(i32, usize)> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_runs() {
        assert_eq!(encode("aaabccdddd"), "3a1b2c4d");
        assert_eq!(encode(""), "");
        assert_eq!(encode("abc"), "1a1b1c");
        assert_eq!(encode(&"z".repeat(12)), "12z");
        assert_eq!(encode("üüx"), "2ü1x");
    }

    #[test]
    fn decode_valid() {
        assert_eq!(decode("3a1b2c4d"), Ok("aaabccdddd".to_string()));
        assert_eq!(decode("12x"), Ok("x".repeat(12)));
        assert_eq!(decode(""), Ok(String::new()));
        assert_eq!(decode("2 1-"), Ok("  -".to_string()));
    }

    #[test]
    fn decode_errors() {
        assert_eq!(decode("a"), Err("missing count before `a`".to_string()));
        assert_eq!(decode("2ab"), Err("missing count before `b`".to_string()));
        assert_eq!(decode("0a"), Err("zero count".to_string()));
        assert_eq!(
            decode("3a12"),
            Err("missing character after `12`".to_string())
        );
        assert_eq!(
            decode("99999999999999999999999x"),
            Err("count too large".to_string())
        );
    }

    #[test]
    fn round_trip() {
        for s in ["aaabccdddd", "x", "abababab", "  ..!!", "mississippi"] {
            assert_eq!(decode(&encode(s)), Ok(s.to_string()));
        }
    }

    #[test]
    fn runs_of_numbers() {
        assert_eq!(runs(&[5, 5, 1, 5, 5, 5]), vec![(5, 2), (1, 1), (5, 3)]);
        assert_eq!(runs(&[]), vec![]);
        assert_eq!(runs(&[7]), vec![(7, 1)]);
        let long = vec![3; 10_000];
        assert_eq!(runs(&long), vec![(3, 10_000)]);
    }
}
