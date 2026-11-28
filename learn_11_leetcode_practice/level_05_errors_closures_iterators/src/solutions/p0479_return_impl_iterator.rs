//! # Problem 0479: Return impl Iterator
//!
//! - **Difficulty:** Medium
//! - **Topics:** returning `impl Iterator<Item = T>`, lazy results, iterators that borrow their input, edition-2024 lifetime capture, infinite iterators with `take`
//! - **Source:** Original · [Rust By Example: `impl Trait`](https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html) · [The Rust Book 10.2: Returning types that implement traits](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)
//!
//! ## Problem
//!
//! Instead of collecting into a `Vec`, a function can return the iterator itself and let the caller
//! decide what to do with it (collect it, sum it, take only a few items...). The concrete type of an
//! adapter chain is long and often unnameable (it contains closures), so the return type is written
//! `impl Iterator<Item = T>`.
//!
//! Implement, **without collecting anything**:
//!
//! - `evens_up_to(n: u32) -> impl Iterator<Item = u32>`: `0, 2, 4, ...` up to `n` inclusive.
//! - `multiples_of(k: u64) -> impl Iterator<Item = u64>`: the **infinite** sequence `k, 2k, 3k, ...`.
//! - `long_words(text: &str, min: usize) -> impl Iterator<Item = &str>`: the whitespace-separated
//!   words with more than `min` characters, in order.
//! - `squares(nums: &[i32]) -> impl Iterator<Item = i32>`: the square of each number.
//! - `numbered_lines(text: &str) -> impl Iterator<Item = String>`: `"1: first line"`,
//!   `"2: second line"`, ...
//!
//! The iterators returned by `long_words`, `squares` and `numbered_lines` **borrow** their input:
//! they can only be used while the input is alive. In edition 2024 a returned `impl Trait`
//! automatically captures the input's lifetime, so no extra annotation is needed (in edition 2021
//! you would have written `+ '_`).
//!
//! ## Examples
//!
//! ```text
//! evens_up_to(7).collect::<Vec<_>>()           -> [0, 2, 4, 6]
//! multiples_of(5).take(3).collect::<Vec<_>>()  -> [5, 10, 15]
//! long_words("a quick brown fox", 3).collect() -> ["quick", "brown"]
//! squares(&[1, -2, 3]).sum::<i32>()            -> 14
//! numbered_lines("x\ny").collect()             -> ["1: x", "2: y"]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints; tests only take a few items of the infinite iterator.
//!
//! ## Hints
//!
//! 1. `(0..=n).step_by(2)` already is an iterator of the right type.
//! 2. `(1..).map(move |i| i * k)` is infinite and lazy; `move` copies `k` into the closure, which
//!    must outlive the function.
//! 3. A closure that uses a parameter (like `min`) inside a returned iterator also needs `move`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build the adapter chain and return it unconsumed. Nothing runs until the caller pulls
//! items, so even an infinite sequence is fine: the caller decides how much of it to use.
//!
//! **Rust concepts:**
//! - `impl Iterator<Item = T>` hides the concrete adapter type while keeping static dispatch (no
//!   boxing).
//! - Closures in a returned iterator must own what they use (`move`), because the function's local
//!   variables and parameters are gone when the iterator runs.
//! - Borrowing iterators carry the lifetime of their input; edition 2024 captures all in-scope
//!   lifetimes in return-position `impl Trait` by default.
//! - `1..` is an unbounded range of integers; it would panic on overflow in debug builds only after
//!   `u64::MAX` items, which no caller can reach.
// @solution-end

pub fn evens_up_to(n: u32) -> impl Iterator<Item = u32> {
    // @solution-start replace: std::iter::from_fn(|| todo!())
    (0..=n).step_by(2)
    // @solution-end
}

pub fn multiples_of(k: u64) -> impl Iterator<Item = u64> {
    // @solution-start replace: std::iter::from_fn(|| todo!())
    (1..).map(move |i| i * k)
    // @solution-end
}

pub fn long_words(text: &str, min: usize) -> impl Iterator<Item = &str> {
    // @solution-start replace: std::iter::from_fn(|| todo!())
    text.split_whitespace()
        .filter(move |w| w.chars().count() > min)
    // @solution-end
}

pub fn squares(nums: &[i32]) -> impl Iterator<Item = i32> {
    // @solution-start replace: std::iter::from_fn(|| todo!())
    nums.iter().map(|x| x * x)
    // @solution-end
}

pub fn numbered_lines(text: &str) -> impl Iterator<Item = String> {
    // @solution-start replace: std::iter::from_fn(|| todo!())
    text.lines()
        .enumerate()
        .map(|(i, line)| format!("{}: {line}", i + 1))
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens() {
        assert_eq!(evens_up_to(7).collect::<Vec<_>>(), vec![0, 2, 4, 6]);
        assert_eq!(evens_up_to(8).collect::<Vec<_>>(), vec![0, 2, 4, 6, 8]);
        assert_eq!(evens_up_to(0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(evens_up_to(100).count(), 51);
        assert_eq!(evens_up_to(u32::MAX).nth(3), Some(6));
    }

    #[test]
    fn infinite_multiples() {
        assert_eq!(multiples_of(5).take(3).collect::<Vec<_>>(), vec![5, 10, 15]);
        assert_eq!(multiples_of(7).nth(99), Some(700));
        assert_eq!(multiples_of(3).take_while(|&m| m < 10).count(), 3);
    }

    #[test]
    fn borrowed_words() {
        let text = String::from("a quick brown fox");
        let words: Vec<&str> = long_words(&text, 3).collect();
        assert_eq!(words, vec!["quick", "brown"]);
        assert_eq!(long_words("tiny bits", 10).count(), 0);
    }

    #[test]
    fn squares_are_lazy_and_reusable_by_the_caller() {
        let nums = [1, -2, 3];
        assert_eq!(squares(&nums).sum::<i32>(), 14);
        assert_eq!(squares(&nums).max(), Some(9));
        assert_eq!(squares(&[]).next(), None);
    }

    #[test]
    fn numbering_lines() {
        assert_eq!(
            numbered_lines("x\ny").collect::<Vec<_>>(),
            vec!["1: x", "2: y"]
        );
        assert_eq!(numbered_lines("").count(), 0);
        assert_eq!(numbered_lines("only").last(), Some("1: only".to_string()));
    }
}
