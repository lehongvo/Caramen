// ==================================================================
// PRACTICE 0107: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0107_fix_value_moved_in_loop.md
// Run the tests:          cargo test practice::p0107
// ==================================================================

//! # Problem 0107: Fix: Value Moved in a Loop
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the borrow checker, error E0382, moves inside loops, `clone`
//! - **Source:** Original · [E0382](https://doc.rust-lang.org/error_codes/E0382.html) · [The Rust Book 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
//!
//! ## Problem
//!
//! A teammate wrote a function that should return a vector with `n` copies of `line`, but it does
//! not compile:
//!
//! ```text
//! pub fn repeat_lines(line: String, n: usize) -> Vec<String> {
//!     let mut out = Vec::new();
//!     for _ in 0..n {
//!         out.push(line);
//!     }
//!     out
//! }
//!
//! error[E0382]: use of moved value: `line`
//!   |     out.push(line);
//!   |              ^^^^ value moved here, in previous iteration of loop
//! ```
//!
//! `push` takes its argument by value, so the first iteration moves `line` into the vector and the
//! second iteration has nothing left to push.
//!
//! Implement the corrected `repeat_lines(line: String, n: usize) -> Vec<String>`: it returns a
//! vector of exactly `n` strings, each equal to `line`. For `n == 0` it returns an empty vector.
//! Keep the signature: the function still takes ownership of `line`.
//!
//! ## Examples
//!
//! ```text
//! Input:  line = "hip", n = 3
//! Output: ["hip", "hip", "hip"]
//!
//! Input:  line = "once", n = 1
//! Output: ["once"]
//!
//! Input:  line = "never", n = 0
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`
//! - `0 <= line.len() <= 100`
//!
//! ## Hints
//!
//! 1. Every element of the result needs its own `String`; `line.clone()` makes an independent copy.
//! 2. You own `line`, so the **last** element can take `line` itself and save one clone.
//! 3. The `vec![value; n]` macro does exactly this for any `Clone` value.

pub fn repeat_lines(line: String, n: usize) -> Vec<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_copies() {
        assert_eq!(
            repeat_lines(String::from("hip"), 3),
            vec!["hip", "hip", "hip"]
        );
    }

    #[test]
    fn example_one_copy() {
        assert_eq!(repeat_lines(String::from("once"), 1), vec!["once"]);
    }

    #[test]
    fn example_zero_copies() {
        assert!(repeat_lines(String::from("never"), 0).is_empty());
    }

    #[test]
    fn copies_are_independent() {
        let mut lines = repeat_lines(String::from("ab"), 2);
        lines[0].push('!');
        assert_eq!(lines, vec!["ab!", "ab"]);
    }

    #[test]
    fn empty_line() {
        assert_eq!(repeat_lines(String::new(), 2), vec!["", ""]);
    }

    #[test]
    fn large_count() {
        let lines = repeat_lines(String::from("row"), 10_000);
        assert_eq!(lines.len(), 10_000);
        assert!(lines.iter().all(|l| l == "row"));
    }
}
