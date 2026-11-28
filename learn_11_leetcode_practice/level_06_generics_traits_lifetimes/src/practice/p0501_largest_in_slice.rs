// ==================================================================
// PRACTICE 0501: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0501_largest_in_slice.md
// Run the tests:          cargo test practice::p0501
// ==================================================================

//! # Problem 0501: Largest Element of a Generic Slice
//!
//! - **Difficulty:** Easy
//! - **Topics:** generic functions, trait bounds `PartialOrd + Copy`, `Option<T>`, `Option<&T>`
//! - **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html)
//!
//! ## Problem
//!
//! Chapter 10 of The Rust Book starts with two copies of the same function: one that finds the
//! largest `i32` and one that finds the largest `char`. Generics let you write it once.
//!
//! Implement two generic functions:
//!
//! - `largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T>` returns a **copy** of the largest
//!   element, or `None` when `list` is empty.
//! - `largest_ref<T: PartialOrd>(list: &[T]) -> Option<&T>` returns a **reference** to the largest
//!   element, or `None` when `list` is empty. It must not require `Copy` or `Clone`, so it also works
//!   for `String` elements.
//!
//! When several elements are equally large, both functions return the **first** one (this matters
//! for `largest_ref`, where you can tell positions apart with `std::ptr::eq`).
//!
//! ## Examples
//!
//! ```text
//! largest(&[34, 50, 25, 100, 65])        => Some(100)
//! largest(&['y', 'm', 'a', 'q'])         => Some('y')
//! largest::<f64>(&[])                    => None
//! largest_ref(&["pear".to_string(), "zebra".to_string(), "apple".to_string()])
//!                                        => Some(&"zebra")
//! ```
//!
//! ## Constraints
//!
//! - `0 <= list.len() <= 10^5`
//! - For floating-point input, the slice contains no `NaN` values.
//!
//! ## Hints
//!
//! 1. `>` comes from the `PartialOrd` trait, so the bound `T: PartialOrd` is what lets you compare.
//! 2. Returning `T` by value out of a `&[T]` needs `Copy`; returning `&T` does not.
//! 3. To keep the first of several equal maxima, replace the current best only when the new element
//!    is strictly greater.

/// Returns a copy of the largest element (the first one on ties), or `None` for an empty slice.
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    todo!("Write your solution here")
}

/// Returns a reference to the largest element (the first one on ties), or `None` for an empty slice.
pub fn largest_ref<T: PartialOrd>(list: &[T]) -> Option<&T> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_of_integers_and_chars() {
        assert_eq!(largest(&[34, 50, 25, 100, 65]), Some(100));
        assert_eq!(largest(&['y', 'm', 'a', 'q']), Some('y'));
        assert_eq!(largest(&[-7_i64]), Some(-7));
    }

    #[test]
    fn largest_of_floats_and_empty() {
        assert_eq!(largest(&[1.5, -2.0, 3.25, 0.0]), Some(3.25));
        assert_eq!(largest::<f64>(&[]), None);
        assert_eq!(largest_ref::<String>(&[]), None);
    }

    #[test]
    fn largest_ref_works_with_strings() {
        let words = vec!["pear".to_string(), "zebra".to_string(), "apple".to_string()];
        assert_eq!(largest_ref(&words), Some(&"zebra".to_string()));
    }

    #[test]
    fn ties_return_the_first_maximum() {
        let values = [3, 9, 1, 9, 2];
        let best = largest_ref(&values).unwrap();
        assert!(std::ptr::eq(best, &values[1]));
        assert_eq!(largest(&values), Some(9));
    }

    #[test]
    fn large_input() {
        let values: Vec<u32> = (0..100_000).map(|i| (i * 7_919) % 100_003).collect();
        let expected = *values.iter().max().unwrap();
        assert_eq!(largest(&values), Some(expected));
        assert_eq!(largest_ref(&values), Some(&expected));
    }
}
