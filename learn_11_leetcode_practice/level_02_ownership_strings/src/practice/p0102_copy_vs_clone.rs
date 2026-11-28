// ==================================================================
// PRACTICE 0102: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0102_copy_vs_clone.md
// Run the tests:          cargo test practice::p0102
// ==================================================================

//! # Problem 0102: Copy vs Clone: Largest Number and Longest Name
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Copy` vs `Clone`, borrowing a slice, `copied()`, `cloned()`, `Option`
//! - **Source:** Original · [The Rust Book 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) · [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) · [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
//!
//! ## Problem
//!
//! Both functions below only **borrow** their input, so the caller keeps ownership of its data.
//! Each one must return an **owned** value taken from the borrowed collection.
//!
//! 1. `largest_number(numbers: &[i32]) -> Option<i32>` returns the largest number, or `None` if the
//!    slice is empty.
//! 2. `longest_name(names: &[String]) -> Option<String>` returns an owned copy of the longest name
//!    (largest `len()`), or `None` if the slice is empty. If several names share the maximum length,
//!    return the **first** one.
//!
//! The point of the exercise: getting an `i32` out of a `&i32` is a cheap bit-for-bit copy because
//! `i32` is `Copy`. Getting a `String` out of a `&String` requires an explicit `.clone()`, which
//! allocates a new heap buffer, because `String` is `Clone` but not `Copy`.
//!
//! ## Examples
//!
//! ```text
//! Input:  numbers = [3, -7, 12, 5]
//! Output: Some(12)
//!
//! Input:  numbers = []
//! Output: None
//!
//! Input:  names = ["Ann", "Bartholomew", "Cy"]
//! Output: Some("Bartholomew")
//!
//! Input:  names = ["Tom", "Eve", "Al"]
//! Output: Some("Tom")        ("Tom" and "Eve" both have length 3; the first one wins)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= numbers.len(), names.len() <= 10^4`
//! - Names contain only ASCII letters, so `len()` equals the number of characters.
//!
//! ## Hints
//!
//! 1. `numbers.iter()` yields `&i32`. `.copied()` turns it into an iterator of `i32`, and `.max()`
//!    already returns an `Option`.
//! 2. For the names, find the longest `&String` first and clone only that one at the end
//!    (`Option<&String>::cloned()` gives `Option<String>`).
//! 3. `Iterator::max_by_key` returns the **last** maximum on ties. Iterating in reverse
//!    (`.rev()`) makes the last maximum of the reversed order the first of the original order.

pub fn largest_number(numbers: &[i32]) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn longest_name(names: &[String]) -> Option<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(names: &[&str]) -> Vec<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn example_largest_number() {
        assert_eq!(largest_number(&[3, -7, 12, 5]), Some(12));
    }

    #[test]
    fn example_empty_numbers() {
        assert_eq!(largest_number(&[]), None);
    }

    #[test]
    fn all_negative_numbers() {
        assert_eq!(largest_number(&[-5, -2, -9]), Some(-2));
    }

    #[test]
    fn example_longest_name() {
        let names = owned(&["Ann", "Bartholomew", "Cy"]);
        assert_eq!(longest_name(&names), Some(String::from("Bartholomew")));
    }

    #[test]
    fn example_tie_returns_first() {
        let names = owned(&["Tom", "Eve", "Al"]);
        assert_eq!(longest_name(&names), Some(String::from("Tom")));
    }

    #[test]
    fn empty_names() {
        assert_eq!(longest_name(&[]), None);
    }

    #[test]
    fn caller_still_owns_the_inputs() {
        let numbers = vec![1, 2, 3];
        let names = owned(&["Zed", "Yolanda"]);
        let best_number = largest_number(&numbers);
        let best_name = longest_name(&names);
        // Both collections are still usable after the calls.
        assert_eq!(numbers.len(), 3);
        assert_eq!(names[1], "Yolanda");
        assert_eq!(best_number, Some(3));
        assert_eq!(best_name.as_deref(), Some("Yolanda"));
    }

    #[test]
    fn large_inputs() {
        let numbers: Vec<i32> = (0..10_000).map(|i| (i * 7919) % 10_007).collect();
        assert_eq!(largest_number(&numbers), numbers.iter().copied().max());
        let mut names: Vec<String> = (0..10_000).map(|i| "a".repeat(i % 50 + 1)).collect();
        names[1234] = "b".repeat(60);
        names[5678] = "c".repeat(60);
        assert_eq!(longest_name(&names), Some("b".repeat(60)));
    }
}
