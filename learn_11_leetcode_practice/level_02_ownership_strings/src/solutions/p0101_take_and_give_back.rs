//! # Problem 0101: Take Ownership and Give It Back
//!
//! - **Difficulty:** Easy
//! - **Topics:** ownership, moving a `String` into a function, returning ownership, `push`
//! - **Source:** Original · [The Rust Book 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
//!
//! ## Problem
//!
//! A function that takes a `String` **by value** becomes its new owner: after the call, the caller
//! can no longer use the variable it passed in (the value was *moved*). If the caller still needs the
//! text, the function must hand ownership back through its return value.
//!
//! Implement `exclaim(text: String, times: usize) -> String`. It receives ownership of `text`,
//! appends exactly `times` exclamation marks (`'!'`) to the **end** of it, and returns that same
//! `String` to the caller.
//!
//! Do not build a second `String`: grow the one you were given. To be allowed to modify an owned
//! parameter, declare it as `mut text: String` (the signature below already does this).
//! If `times` is `0`, return `text` unchanged.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello", times = 1
//! Output: "hello!"
//!
//! Input:  text = "Rust", times = 3
//! Output: "Rust!!!"
//!
//! Input:  text = "", times = 2
//! Output: "!!"
//!
//! Caller side:
//!     let s = String::from("hi");
//!     let s = exclaim(s, 2);   // `s` was moved in, and a String comes back out
//!     assert_eq!(s, "hi!!");
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 1000`
//! - `0 <= times <= 1000`
//!
//! ## Hints
//!
//! 1. `mut` on a parameter only makes the function's own binding mutable; the caller does not need
//!    a mutable variable to pass the `String` in.
//! 2. `String::push` appends one `char`; call it in a `for _ in 0..times` loop.
//! 3. The last expression of the function body (without a `;`) is returned, which moves the
//!    `String` back to the caller.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the function owns `text`, so it may change it freely. Push one `'!'` per iteration and
//! return `text` as the final expression. No copy of the text is ever made: the same heap buffer
//! travels into the function and back out.
//!
//! **Rust concepts:**
//! - Passing a `String` by value **moves** it: the caller's variable becomes unusable until it is
//!   assigned again (`let s = exclaim(s, 2);` creates a new binding with the returned value).
//! - `mut text: String` is a mutable *binding*; mutability belongs to the binding, not to the value.
//! - `push(char)` may reallocate the buffer when it grows, but it is still the same owned `String`.
//! - Returning a value is also a move, so "take ownership and give it back" costs nothing.
// @solution-end

pub fn exclaim(mut text: String, times: usize) -> String {
    // @solution-start
    for _ in 0..times {
        text.push('!');
    }
    text
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_mark() {
        assert_eq!(exclaim(String::from("hello"), 1), "hello!");
    }

    #[test]
    fn example_three_marks() {
        assert_eq!(exclaim(String::from("Rust"), 3), "Rust!!!");
    }

    #[test]
    fn example_empty_text() {
        assert_eq!(exclaim(String::new(), 2), "!!");
    }

    #[test]
    fn zero_times_returns_text_unchanged() {
        assert_eq!(exclaim(String::from("calm"), 0), "calm");
    }

    #[test]
    fn caller_gets_ownership_back_and_can_reuse_it() {
        let greeting = String::from("hi");
        let greeting = exclaim(greeting, 2);
        let greeting = exclaim(greeting, 1);
        assert_eq!(greeting, "hi!!!");
    }

    #[test]
    fn non_ascii_text_is_kept() {
        assert_eq!(exclaim(String::from("Zürich"), 2), "Zürich!!");
    }

    #[test]
    fn large_count() {
        let result = exclaim(String::from("x"), 1000);
        assert_eq!(result.len(), 1001);
        assert!(result.starts_with("x!"));
        assert!(result.ends_with("!!!"));
    }
}
